// src/domain/vault.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptedSecret {
    pub id: String,
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub salt: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub nonce: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DecryptedSecret {
    pub id: String,
    pub payload: Vec<u8>,
    pub metadata: Option<String>,
}