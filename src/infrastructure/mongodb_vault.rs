// src/infrastructure/mongodb_vault.rs
use crate::domain::VaultRepository;
use crate::domain::vault::EncryptedSecret;
use crate::errors::{AppError, AppResult};
use async_trait::async_trait;
use mongodb::{Database, bson::doc};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;

pub struct MongoVaultRepository {
    db: Arc<Database>,
    collection_name: String,
}

impl MongoVaultRepository {
    pub fn new(db: Arc<Database>) -> Self {
        Self {
            db,
            collection_name: "vaults".to_string(),
        }
    }
}

#[async_trait]
impl VaultRepository for MongoVaultRepository {
    async fn get_secret_by_id(&self, id: &str) -> AppResult<Option<EncryptedSecret>> {
        let collection = self.db.collection::<EncryptedSecret>(&self.collection_name);
        let filter = doc! { "id": id };

        let result = timeout(Duration::from_secs(5), collection.find_one(filter))
            .await
            .map_err(|_| AppError::TimeoutError)?
            .map_err(AppError::DatabaseError)?;

        Ok(result)
    }
}
