// src/application/use_cases/rotate_key.rs
use std::sync::Arc;

use crate::{
    application::use_cases::generate_key_pair::{GenerateKeyPairInput, GenerateKeyPairUseCase},
    domain::keys::{
        models::{KeyAlgorithm, KeyPairEntity, ServiceId},
        repository::KeyRepository,
    },
    errors::{AppError, AppResult},
    infrastructure::crypto::kms_service::KmsCryptoService,
};

pub struct RotateKeyInput {
    pub service_id: ServiceId,
    pub algorithm: KeyAlgorithm,
}

pub struct RotateKeyUseCase<R>
where
    R: KeyRepository + Send + Sync,
{
    generate_key_pair_use_case: GenerateKeyPairUseCase<R>,
    key_repo: Arc<R>,
}

impl<R> RotateKeyUseCase<R>
where
    R: KeyRepository + Send + Sync,
{
    pub fn new(key_repo: Arc<R>, crypto_service: Arc<KmsCryptoService>) -> Self {
        let generate_key_pair_use_case =
            GenerateKeyPairUseCase::new(Arc::clone(&key_repo), crypto_service);
        Self {
            generate_key_pair_use_case,
            key_repo,
        }
    }

    pub async fn execute(&self, input: RotateKeyInput) -> AppResult<KeyPairEntity> {
        let active_key = self
            .key_repo
            .get_active_key(&input.service_id, input.algorithm)
            .await?
            .ok_or_else(|| {
                AppError::NotFound(format!(
                    "Cannot rotate key: No active key exists for service '{}' with algorithm '{:?}'",
                    input.service_id.0, input.algorithm
                ))
            })?;

        self.generate_key_pair_use_case
            .execute(GenerateKeyPairInput {
                service_id: input.service_id,
                algorithm: input.algorithm,
                purpose: active_key.purpose,
            })
            .await
    }
}
