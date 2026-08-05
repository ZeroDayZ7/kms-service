// src/application/use_cases/unlock_secret.rs
use crate::{
    domain::{
        VaultRepository,
        crypto::{EncryptedPrivateKey, KmsCryptoService},
        ports::decoder::Decoder,
        vault::DecryptedSecret,
    },
    errors::{AppError, AppResult},
};
use std::sync::Arc;

pub struct UnlockSecretUseCase<R, C, D>
where
    R: VaultRepository,
    C: KmsCryptoService,
    D: Decoder<DecryptedSecret>,
{
    repo: Arc<R>,
    crypto: Arc<C>,
    decoder: Arc<D>,
}

impl<R, C, D> UnlockSecretUseCase<R, C, D>
where
    R: VaultRepository,
    C: KmsCryptoService,
    D: Decoder<DecryptedSecret>,
{
    pub fn new(repo: Arc<R>, crypto: Arc<C>, decoder: Arc<D>) -> Self {
        Self {
            repo,
            crypto,
            decoder,
        }
    }

    pub async fn execute(&self, id: &str) -> AppResult<DecryptedSecret> {
        let encrypted = self
            .repo
            .get_secret_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Secret {} not found", id)))?;

        let payload = EncryptedPrivateKey {
            ciphertext: encrypted.data,
            nonce: encrypted.nonce,
        };

        let decrypted_bytes = self.crypto.decrypt_private_key(&payload)?;
        let secret = self.decoder.decode(&decrypted_bytes)?;

        Ok(secret)
    }
}
