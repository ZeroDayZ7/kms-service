use chrono::Utc;
use std::sync::Arc;

use crate::{
    domain::{
        crypto::KmsCryptoService,
        keys::{
            models::{KeyAlgorithm, KeyPairEntity, KeyPurpose, ServiceId},
            repository::KeyRepository,
        },
    },
    errors::AppResult,
};

pub struct GenerateKeyPairInput {
    pub service_id: ServiceId,
    pub algorithm: KeyAlgorithm,
    pub purpose: KeyPurpose,
}

pub struct GenerateKeyPairUseCase<R> {
    key_repo: Arc<R>,
    crypto_service: Arc<dyn KmsCryptoService + Send + Sync>,
}

impl<R> GenerateKeyPairUseCase<R>
where
    R: KeyRepository,
{
    pub fn new(key_repo: Arc<R>, crypto_service: Arc<dyn KmsCryptoService + Send + Sync>) -> Self {
        Self {
            key_repo,
            crypto_service,
        }
    }

    pub async fn execute(&self, input: GenerateKeyPairInput) -> AppResult<KeyPairEntity> {
        let current_key = self
            .key_repo
            .get_active_key(&input.service_id, input.algorithm)
            .await?;

        let version = match current_key {
            Some(ref key) => {
                self.key_repo
                    .deactivate_keys_for_service(&input.service_id, input.algorithm)
                    .await?;
                key.version + 1
            }
            None => 1,
        };

        let generated_pair = match input.algorithm {
            KeyAlgorithm::Ed25519 => self.crypto_service.generate_ed25519_keypair()?,
            KeyAlgorithm::X25519 => self.crypto_service.generate_x25519_keypair()?,
        };

        let public_key_pem = generated_pair.public_key_pem.clone();

        let encrypted_private_key = self
            .crypto_service
            .encrypt_private_key(&generated_pair.private_key_bytes)?;

        let entity = KeyPairEntity {
            id: uuid::Uuid::now_v7(),
            service_id: input.service_id,
            algorithm: input.algorithm,
            purpose: input.purpose,
            public_key_pem,
            encrypted_private_key,
            version,
            is_active: true,
            created_at: Utc::now(),
            expires_at: None,
        };

        self.key_repo.save_key(&entity).await?;

        Ok(entity)
    }
}
