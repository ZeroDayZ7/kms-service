use std::sync::Arc;

use crate::domain::crypto::KmsCryptoService;
use crate::domain::keys::repository::KeyRepository;
use crate::errors::{AppError, AppResult};

pub struct RewrapKeysInput {
    pub target_master_version: i32,
}

pub struct RewrapKeysUseCase<R>
where
    R: KeyRepository + Send + Sync,
{
    key_repo: Arc<R>,
    crypto_service: Arc<dyn KmsCryptoService + Send + Sync>,
}

impl<R> RewrapKeysUseCase<R>
where
    R: KeyRepository + Send + Sync,
{
    pub fn new(key_repo: Arc<R>, crypto_service: Arc<dyn KmsCryptoService + Send + Sync>) -> Self {
        Self {
            key_repo,
            crypto_service,
        }
    }

    pub async fn execute(&self, input: RewrapKeysInput) -> AppResult<()> {
        // Validate target matches current master key version
        let current = self.crypto_service.current_master_key_version();
        if current != input.target_master_version {
            return Err(AppError::BadRequest(format!(
                "Target master key version {} does not match current version {}",
                input.target_master_version, current
            )));
        }

        let keys = self.key_repo.get_all_keys().await?;

        for key in keys {
            // Decrypt with the key's stored master_key_version
            let decrypted = self.crypto_service.decrypt_private_key(&key.encrypted_private_key)?;

            // Encrypt with current master key
            let reencrypted = self.crypto_service.encrypt_private_key(&decrypted)?;

            // Persist updated encrypted blob
            self.key_repo
                .update_encrypted_key(&key.id, reencrypted)
                .await?;
        }

        Ok(())
    }
}
