// src/domain/keys/repository.rs
use crate::{
    domain::keys::models::{KeyAlgorithm, KeyPairEntity, ServiceId},
    errors::AppResult,
};
use async_trait::async_trait;

#[async_trait]
pub trait KeyRepository: Send + Sync {
    async fn save_key(&self, key: &KeyPairEntity) -> AppResult<()>;
    async fn get_active_key(&self, service_id: &ServiceId, algo: KeyAlgorithm) -> AppResult<Option<KeyPairEntity>>;
    async fn get_all_active_public_keys(&self) -> AppResult<Vec<KeyPairEntity>>;
    async fn deactivate_keys_for_service(&self, service_id: &ServiceId, algo: KeyAlgorithm) -> AppResult<()>;
}