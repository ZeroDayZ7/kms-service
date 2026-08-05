use std::sync::Arc;

use crate::{
    domain::{
        crypto::KmsCryptoService,
        keys::{
            models::{KeyAlgorithm, ServiceId},
            repository::KeyRepository,
        },
    },
    errors::{AppError, AppResult},
};

pub struct GetPrivateKeyInput {
    pub service_id: ServiceId,
    pub algorithm: KeyAlgorithm,
}

pub struct GetPrivateKeyOutput {
    pub service_id: ServiceId,
    pub algorithm: KeyAlgorithm,
    pub version: u32,
    pub private_key_bytes: Vec<u8>,
}

pub struct GetPrivateKeyUseCase<R> {
    key_repo: Arc<R>,
    crypto_service: Arc<dyn KmsCryptoService + Send + Sync>,
}

impl<R> GetPrivateKeyUseCase<R>
where
    R: KeyRepository,
{
    pub fn new(key_repo: Arc<R>, crypto_service: Arc<dyn KmsCryptoService + Send + Sync>) -> Self {
        Self {
            key_repo,
            crypto_service,
        }
    }

    pub async fn execute(&self, input: GetPrivateKeyInput) -> AppResult<GetPrivateKeyOutput> {
        let active_key = self
            .key_repo
            .get_active_key(&input.service_id, input.algorithm)
            .await?
            .ok_or_else(|| {
                AppError::NotFound(format!(
                    "No active key found for service {} and algorithm {:?}",
                    input.service_id.0, input.algorithm
                ))
            })?;

        let decrypted_private_key = self
            .crypto_service
            .decrypt_private_key(&active_key.encrypted_private_key)?;

        Ok(GetPrivateKeyOutput {
            service_id: active_key.service_id,
            algorithm: active_key.algorithm,
            version: active_key.version,
            private_key_bytes: decrypted_private_key,
        })
    }
}
