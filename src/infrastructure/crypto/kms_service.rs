use aes_gcm::{
    aead::{Aead, KeyInit, OsRng, rand_core::RngCore},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use ed25519_dalek::{SigningKey, pkcs8::EncodePublicKey};
use pkcs8::LineEnding;
use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret};

use crate::{
    config::crypto::CryptoSettings,
    domain::crypto::{EncryptedPrivateKey, KmsCryptoService as KmsCryptoServiceTrait, RawKeyPair},
    errors::{AppError, AppResult},
};

const NONCE_LEN: usize = 12;

pub struct KmsCryptoService {
    master_key: [u8; 32],
}

impl KmsCryptoService {
    pub fn new(settings: &CryptoSettings) -> AppResult<Self> {
        let decoded_key = BASE64
            .decode(&settings.master_key_b64.0)
            .map_err(|e| AppError::CryptoError(format!("Invalid Master Key Base64: {e}")))?;

        let master_key: [u8; 32] = decoded_key
            .try_into()
            .map_err(|_| AppError::CryptoError("Master key must be exactly 32 bytes".into()))?;

        Ok(Self { master_key })
    }
}

impl KmsCryptoServiceTrait for KmsCryptoService {
    fn generate_ed25519_keypair(&self) -> AppResult<RawKeyPair> {
        let mut rng = OsRng;
        let signing_key = SigningKey::generate(&mut rng);
        let verifying_key = signing_key.verifying_key();

        let public_key_pem = verifying_key
            .to_public_key_pem(LineEnding::LF)
            .map_err(|e| AppError::CryptoError(format!("Failed to encode Ed25519 public key to PEM: {e}")))?;

        Ok(RawKeyPair {
            public_key_pem,
            private_key_bytes: signing_key.to_bytes().to_vec(),
        })
    }

    fn generate_x25519_keypair(&self) -> AppResult<RawKeyPair> {
        let rng = OsRng;
        let secret = StaticSecret::random_from_rng(rng);
        let public = X25519PublicKey::from(&secret);

        let public_key_pem = pem::encode(&pem::Pem::new("X25519 PUBLIC KEY", public.as_bytes().to_vec()));

        Ok(RawKeyPair {
            public_key_pem,
            private_key_bytes: secret.to_bytes().to_vec(),
        })
    }

    fn encrypt_private_key(&self, private_key: &[u8]) -> AppResult<EncryptedPrivateKey> {
        let mut nonce_bytes = [0u8; NONCE_LEN];
        let mut rng = OsRng;

        rng.try_fill_bytes(&mut nonce_bytes)
            .map_err(|e| AppError::CryptoError(format!("RNG error: {e}")))?;

        let cipher = Aes256Gcm::new_from_slice(&self.master_key)
            .map_err(|e| AppError::CryptoError(format!("Cipher initialization error: {e}")))?;

        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, private_key)
            .map_err(|e| AppError::CryptoError(format!("Envelope encryption failed: {e}")))?;

        Ok(EncryptedPrivateKey {
            ciphertext,
            nonce: nonce_bytes.to_vec(),
        })
    }

    fn decrypt_private_key(&self, encrypted: &EncryptedPrivateKey) -> AppResult<Vec<u8>> {
        if encrypted.nonce.len() != NONCE_LEN {
            return Err(AppError::CryptoError(format!(
                "Invalid nonce length: expected {NONCE_LEN} bytes, got {}",
                encrypted.nonce.len()
            )));
        }

        let cipher = Aes256Gcm::new_from_slice(&self.master_key)
            .map_err(|e| AppError::CryptoError(format!("Cipher initialization error: {e}")))?;

        let nonce = Nonce::from_slice(&encrypted.nonce);

        let decrypted = cipher
            .decrypt(nonce, encrypted.ciphertext.as_slice())
            .map_err(|_| AppError::CryptoError("Envelope decryption failed: invalid key or tampered data".into()))?;

        Ok(decrypted)
    }
}