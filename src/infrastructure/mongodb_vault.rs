// Copyright 2026 ZeroDayZ7
// Licensed under the Apache License, Version 2.0
// See LICENSE file for details.

use crate::domain::VaultRepository;
use crate::domain::vault::EncryptedCV;
use crate::errors::AppError;
use crate::errors::AppResult;
use mongodb::{Database, bson::doc};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;
use async_trait::async_trait;

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
    async fn get_cv_by_id(&self, id: &str) -> AppResult<Option<EncryptedCV>> {
        let collection = self.db.collection::<EncryptedCV>(&self.collection_name);
        let filter = doc! { "id": id };

        let result = timeout(Duration::from_secs(5), collection.find_one(filter))
            .await
            .map_err(|_| AppError::TimeoutError)?
            .map_err(AppError::DatabaseError)?;

        Ok(result)
    }
}