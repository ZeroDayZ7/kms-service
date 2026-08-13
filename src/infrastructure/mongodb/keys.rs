use std::sync::Arc;

use mongodb::{
    IndexModel,
    bson::{Binary, DateTime as BsonDateTime, doc, spec::BinarySubtype},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    domain::{
        crypto::EncryptedPrivateKey,
        keys::{
            models::{KeyAlgorithm, KeyPairEntity, KeyPurpose, ServiceId},
            repository::KeyRepository,
        },
    },
    errors::{AppError, AppResult},
};

#[derive(Debug, Serialize, Deserialize)]
struct MongoKeyPairDocument {
    pub id: String,
    pub service_id: String,
    pub algorithm: String,
    pub purpose: String,
    pub public_key_pem: String,
    pub encrypted_private_key: Binary,
    pub nonce: Binary,
    pub version: i32,
    pub master_key_version: i32,
    pub status: String,
    pub deprecated_valid_until: Option<BsonDateTime>,
    pub created_at: BsonDateTime,
    pub expires_at: Option<BsonDateTime>,
}

pub struct MongoKeyRepository {
    db: Arc<mongodb::Database>,
}

impl MongoKeyRepository {
    pub fn new(db: Arc<mongodb::Database>) -> Self {
        Self { db }
    }

    fn collection(&self) -> mongodb::Collection<MongoKeyPairDocument> {
        self.db.collection("key_pairs")
    }

    pub async fn ensure_indexes(&self) -> AppResult<()> {
        let active_key_index = IndexModel::builder()
            .keys(doc! {
                "service_id": 1,
                "algorithm": 1,
                "status": 1
            })
            .build();

        self.collection().create_index(active_key_index).await?;

        Ok(())
    }
}

impl KeyRepository for MongoKeyRepository {
    async fn save_key(&self, key_pair: &KeyPairEntity) -> AppResult<()> {
        let doc = MongoKeyPairDocument {
            id: key_pair.id.to_string(),
            service_id: key_pair.service_id.0.clone(),
            algorithm: format!("{:?}", key_pair.algorithm),
            purpose: format!("{:?}", key_pair.purpose),
            public_key_pem: key_pair.public_key_pem.clone(),
            encrypted_private_key: Binary {
                subtype: BinarySubtype::Generic,
                bytes: key_pair.encrypted_private_key.ciphertext.clone(),
            },
            nonce: Binary {
                subtype: BinarySubtype::Generic,
                bytes: key_pair.encrypted_private_key.nonce.clone(),
            },
            version: key_pair.version as i32,
            master_key_version: key_pair.encrypted_private_key.master_key_version,
            status: match &key_pair.status {
                crate::domain::keys::models::KeyStatus::Active => "Active".to_string(),
                crate::domain::keys::models::KeyStatus::Revoked => "Revoked".to_string(),
                crate::domain::keys::models::KeyStatus::Compromised => "Compromised".to_string(),
                crate::domain::keys::models::KeyStatus::Deprecated { valid_until: _ } => {
                    "Deprecated".to_string()
                }
            },
            deprecated_valid_until: match &key_pair.status {
                crate::domain::keys::models::KeyStatus::Deprecated { valid_until } => {
                    Some((*valid_until).into())
                }
                _ => None,
            },
            created_at: key_pair.created_at.into(),
            expires_at: key_pair.expires_at.map(Into::into),
        };

        self.collection().insert_one(doc).await?;

        Ok(())
    }

    async fn get_active_key(
        &self,
        service_id: &ServiceId,
        algorithm: KeyAlgorithm,
    ) -> AppResult<Option<KeyPairEntity>> {
        let filter = doc! {
            "service_id": &service_id.0,
            "algorithm": format!("{:?}", algorithm),
            "status": "Active"
        };

        let result = self.collection().find_one(filter).await?;

        match result {
            Some(doc) => Ok(Some(map_doc_to_entity(doc)?)),
            None => Ok(None),
        }
    }

    async fn get_all_active_public_keys(&self) -> AppResult<Vec<KeyPairEntity>> {
        let filter = doc! { "status": "Active" };
        let mut cursor = self.collection().find(filter).await?;

        let mut keys = Vec::new();
        while cursor.advance().await? {
            let doc = cursor.deserialize_current()?;
            keys.push(map_doc_to_entity(doc)?);
        }

        Ok(keys)
    }

    async fn deactivate_keys_for_service(
        &self,
        service_id: &ServiceId,
        algorithm: KeyAlgorithm,
    ) -> AppResult<()> {
        let filter = doc! {
            "service_id": &service_id.0,
            "algorithm": format!("{:?}", algorithm),
            "status": "Active"
        };
        let update =
            doc! { "$set": { "status": "Revoked", "deprecated_valid_until": bson::Bson::Null } };

        self.collection().update_many(filter, update).await?;

        Ok(())
    }

    async fn update_key_status(
        &self,
        key_id: &Uuid,
        status: crate::domain::keys::models::KeyStatus,
        deprecated_until: Option<chrono::DateTime<chrono::Utc>>,
    ) -> AppResult<()> {
        let filter = doc! { "id": key_id.to_string() };

        let status_str = match status {
            crate::domain::keys::models::KeyStatus::Active => "Active",
            crate::domain::keys::models::KeyStatus::Revoked => "Revoked",
            crate::domain::keys::models::KeyStatus::Compromised => "Compromised",
            crate::domain::keys::models::KeyStatus::Deprecated { .. } => "Deprecated",
        };

        let update_doc = if let Some(dt) = deprecated_until {
            doc! { "$set": { "status": status_str, "deprecated_valid_until": BsonDateTime::from_chrono(dt) } }
        } else {
            doc! { "$set": { "status": status_str, "deprecated_valid_until": bson::Bson::Null } }
        };

        self.collection().update_one(filter, update_doc).await?;
        Ok(())
    }

    async fn get_deprecated_keys_expired(
        &self,
        now: chrono::DateTime<chrono::Utc>,
    ) -> AppResult<Vec<KeyPairEntity>> {
        let filter = doc! {
            "status": "Deprecated",
            "deprecated_valid_until": { "$lte": BsonDateTime::from_chrono(now) }
        };

        let mut cursor = self.collection().find(filter).await?;
        let mut keys = Vec::new();
        while cursor.advance().await? {
            let doc = cursor.deserialize_current()?;
            keys.push(map_doc_to_entity(doc)?);
        }

        Ok(keys)
    }

    async fn get_active_or_valid_deprecated_key(
        &self,
        service_id: &ServiceId,
        algorithm: KeyAlgorithm,
        now: chrono::DateTime<chrono::Utc>,
    ) -> AppResult<Option<KeyPairEntity>> {
        let filter = doc! {
            "$or": [
                { "status": "Active" },
                { "status": "Deprecated", "deprecated_valid_until": { "$gt": BsonDateTime::from_chrono(now) } }
            ],
            "service_id": &service_id.0,
            "algorithm": format!("{:?}", algorithm),
        };

        let result = self.collection().find_one(filter).await?;
        match result {
            Some(doc) => Ok(Some(map_doc_to_entity(doc)?)),
            None => Ok(None),
        }
    }
}

fn map_doc_to_entity(doc: MongoKeyPairDocument) -> AppResult<KeyPairEntity> {
    let id = Uuid::parse_str(&doc.id)
        .map_err(|e| AppError::Internal(format!("Invalid UUID in database: {}", e)))?;

    let algorithm = match doc.algorithm.as_str() {
        "Ed25519" => KeyAlgorithm::Ed25519,
        "X25519" => KeyAlgorithm::X25519,
        "AES256GCM" => KeyAlgorithm::AES256GCM,
        "HmacSha256" => KeyAlgorithm::HmacSha256,
        other => {
            return Err(AppError::Internal(format!(
                "Unknown algorithm in database: {}",
                other
            )));
        }
    };

    let purpose = match doc.purpose.as_str() {
        "Signing" => KeyPurpose::Signing,
        "Encryption" => KeyPurpose::Encryption,
        "Authentication" => KeyPurpose::Authentication,
        other => {
            return Err(AppError::Internal(format!(
                "Unknown purpose in database: {}",
                other
            )));
        }
    };

    let created_at = doc.created_at.to_chrono();
    let expires_at = doc.expires_at.map(|dt| dt.to_chrono());

    Ok(KeyPairEntity {
        id,
        service_id: ServiceId(doc.service_id),
        algorithm,
        purpose,
        public_key_pem: doc.public_key_pem,
        encrypted_private_key: EncryptedPrivateKey {
            ciphertext: doc.encrypted_private_key.bytes,
            nonce: doc.nonce.bytes,
            master_key_version: doc.master_key_version,
        },
        version: doc.version as u32,
        status: match doc.status.as_str() {
            "Active" => crate::domain::keys::models::KeyStatus::Active,
            "Revoked" => crate::domain::keys::models::KeyStatus::Revoked,
            "Compromised" => crate::domain::keys::models::KeyStatus::Compromised,
            "Deprecated" => {
                if let Some(dt) = doc.deprecated_valid_until {
                    crate::domain::keys::models::KeyStatus::Deprecated {
                        valid_until: dt.to_chrono(),
                    }
                } else {
                    crate::domain::keys::models::KeyStatus::Deprecated {
                        valid_until: chrono::Utc::now(),
                    }
                }
            }
            _ => crate::domain::keys::models::KeyStatus::Revoked,
        },
        created_at,
        expires_at,
    })
}
