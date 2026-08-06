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
    pub is_active: bool,
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
                "is_active": 1
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
            is_active: key_pair.is_active,
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
            "is_active": true
        };

        let result = self.collection().find_one(filter).await?;

        match result {
            Some(doc) => Ok(Some(map_doc_to_entity(doc)?)),
            None => Ok(None),
        }
    }

    async fn get_all_active_public_keys(&self) -> AppResult<Vec<KeyPairEntity>> {
        let filter = doc! { "is_active": true };
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
            "is_active": true
        };
        let update = doc! { "$set": { "is_active": false } };

        self.collection().update_many(filter, update).await?;

        Ok(())
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
        },
        version: doc.version as u32,
        is_active: doc.is_active,
        created_at,
        expires_at,
    })
}