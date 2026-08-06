// src/domain/crypto.rs
use crate::errors::AppResult;
use serde::{Deserialize, Serialize};
use zeroize::ZeroizeOnDrop;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum KeyAlgorithm {
    Ed25519,
    X25519,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum KeyPurpose {
    Signing,
    Encryption,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedPrivateKey {
    pub ciphertext: Vec<u8>,
    pub nonce: Vec<u8>,
}

#[derive(ZeroizeOnDrop)]
pub struct RawKeyPair {
    pub public_key_pem: String,
    pub private_key_bytes: Vec<u8>,
}

pub trait KmsCryptoService: Send + Sync {
    fn generate_ed25519_keypair(&self) -> AppResult<RawKeyPair>;
    fn generate_x25519_keypair(&self) -> AppResult<RawKeyPair>;
    fn encrypt_private_key(&self, private_key: &[u8]) -> AppResult<EncryptedPrivateKey>;
    fn decrypt_private_key(&self, encrypted: &EncryptedPrivateKey) -> AppResult<Vec<u8>>;
}
