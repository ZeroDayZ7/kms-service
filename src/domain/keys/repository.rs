// src/domain/keys/repository.rs
use crate::{
    domain::keys::models::{KeyAlgorithm, KeyPairEntity, ServiceId},
    errors::AppResult,
};

pub trait KeyRepository: Send + Sync {
    fn save_key(
        &self,
        key: &KeyPairEntity,
    ) -> impl std::future::Future<Output = AppResult<()>> + Send;
    fn get_active_key(
        &self,
        service_id: &ServiceId,
        algo: KeyAlgorithm,
    ) -> impl std::future::Future<Output = AppResult<Option<KeyPairEntity>>> + Send;
    fn get_all_active_public_keys(
        &self,
    ) -> impl std::future::Future<Output = AppResult<Vec<KeyPairEntity>>> + Send;
    fn deactivate_keys_for_service(
        &self,
        service_id: &ServiceId,
        algo: KeyAlgorithm,
    ) -> impl std::future::Future<Output = AppResult<()>> + Send;
}
