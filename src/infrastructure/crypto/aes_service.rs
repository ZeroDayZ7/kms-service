// Copyright 2026 ZeroDayZ7
// Licensed under the Apache License, Version 2.0
// See LICENSE file for details.

use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit, OsRng, rand_core::RngCore},
};
use argon2::{Algorithm, Argon2, Params, Version};

use crate::{
    config::crypto::CryptoSettings,
    domain::crypto::{CryptoService, EncryptedPayload},
    errors::{AppError, AppResult},
};

// Stałe kryptograficzne zaszyte na sztywno
const SALT_LEN: usize = 16;  // 128-bit salt (bezpieczne dla Argon2id)
const NONCE_LEN: usize = 12; // 96-bit nonce (standard dla AES-256-GCM)

pub struct AesCryptoService {
    settings: CryptoSettings,
}

impl AesCryptoService {
    pub fn new(settings: CryptoSettings) -> Self {
        Self { settings }
    }

    fn derive_key(&self, password: &str, salt: &[u8]) -> AppResult<[u8; 32]> {
        let mut key = [0u8; 32];
        let password_with_pepper = format!("{}{}", password, self.settings.secret_key);

        let params = Params::new(
            self.settings.argon2_m_cost,
            self.settings.argon2_t_cost,
            self.settings.argon2_p_cost,
            None,
        )
        .map_err(|e| AppError::CryptoError(format!("Argon2 params invalid: {e}")))?;

        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

        argon2
            .hash_password_into(password_with_pepper.as_bytes(), salt, &mut key)
            .map_err(|e| AppError::CryptoError(format!("Argon2 derivation failed: {e}")))?;

        Ok(key)
    }
}

impl CryptoService for AesCryptoService {
    fn encrypt(&self, data: &[u8], password: &str) -> AppResult<EncryptedPayload> {
        let mut salt = vec![0u8; SALT_LEN];
        let mut nonce_bytes = vec![0u8; NONCE_LEN];

        let mut rng = OsRng;
        rng.try_fill_bytes(&mut salt)
            .map_err(|e| AppError::CryptoError(format!("RNG error: {e}")))?;

        rng.try_fill_bytes(&mut nonce_bytes)
            .map_err(|e| AppError::CryptoError(format!("RNG error: {e}")))?;

        let key = self.derive_key(password, &salt)?;

        let cipher = Aes256Gcm::new_from_slice(&key)
            .map_err(|e| AppError::CryptoError(format!("Cipher initialization error: {e}")))?;

        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, data)
            .map_err(|e| AppError::CryptoError(format!("Encryption failed: {e}")))?;

        Ok(EncryptedPayload {
            ciphertext,
            salt,
            nonce: nonce_bytes,
        })
    }

    fn decrypt(&self, payload: &EncryptedPayload, password: &str) -> AppResult<Vec<u8>> {
        if payload.salt.len() != SALT_LEN {
            return Err(AppError::CryptoError(format!(
                "Invalid salt length: expected {SALT_LEN} bytes, got {}",
                payload.salt.len()
            )));
        }

        if payload.nonce.len() != NONCE_LEN {
            return Err(AppError::CryptoError(format!(
                "Invalid nonce length: expected {NONCE_LEN} bytes, got {}",
                payload.nonce.len()
            )));
        }

        let key = self.derive_key(password, &payload.salt)?;

        let cipher = Aes256Gcm::new_from_slice(&key)
            .map_err(|e| AppError::CryptoError(format!("Cipher initialization error: {e}")))?;

        let nonce = Nonce::from_slice(&payload.nonce);

        let decrypted = cipher
            .decrypt(nonce, payload.ciphertext.as_ref())
            .map_err(|_| {
                AppError::CryptoError("Decryption failed: check password or data integrity".into())
            })?;

        Ok(decrypted)
    }
}