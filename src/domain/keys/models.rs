// src/domain/keys/models.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;
use zeroize::ZeroizeOnDrop;

pub use crate::domain::crypto::{EncryptedPrivateKey, KeyAlgorithm, KeyPurpose};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ServiceId(pub String);

impl fmt::Display for ServiceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for ServiceId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

#[derive(ZeroizeOnDrop)]
pub struct RawKeyPair {
    pub public_key_pem: String,
    pub private_key_bytes: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyPairEntity {
    pub id: Uuid,
    pub service_id: ServiceId,
    pub algorithm: KeyAlgorithm,
    pub purpose: KeyPurpose,
    pub public_key_pem: String,
    pub encrypted_private_key: EncryptedPrivateKey,
    pub version: u32,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}
