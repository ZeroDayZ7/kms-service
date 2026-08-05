// Copyright 2026 ZeroDayZ7
// Licensed under the Apache License, Version 2.0
// See LICENSE file for details.

use std::sync::Arc;
use crate::{
    domain::{
        VaultRepository,
        crypto::{CryptoService, EncryptedPayload},
        ports::decoder::Decoder,
        vault::DecryptedCV,
    },
    errors::{AppError, AppResult},
};

pub struct UnlockCvUseCase<R, C, D>
where
    R: VaultRepository,
    C: CryptoService,
    D: Decoder<DecryptedCV>,
{
    repo: Arc<R>,
    crypto: Arc<C>,
    decoder: Arc<D>,
}

impl<R, C, D> UnlockCvUseCase<R, C, D>
where
    R: VaultRepository,
    C: CryptoService,
    D: Decoder<DecryptedCV>,
{
    pub fn new(
        repo: Arc<R>,
        crypto: Arc<C>,
        decoder: Arc<D>,
    ) -> Self {
        Self {
            repo,
            crypto,
            decoder,
        }
    }

    pub async fn execute(&self, id: &str, key: &str) -> AppResult<DecryptedCV> {
        let encrypted = self
            .repo
            .get_cv_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("CV {} not found", id)))?;

        let payload = EncryptedPayload {
            ciphertext: encrypted.data,
            salt: encrypted.salt,
            nonce: encrypted.nonce,
        };

        let decrypted_bytes = self.crypto.decrypt(&payload, key)?;
        let cv = self.decoder.decode(&decrypted_bytes)?;

        Ok(cv)
    }
}