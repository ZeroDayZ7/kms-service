// src/config/crypto.rs
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use serde::{Deserialize, Deserializer};
use std::ops::Deref;

// --- TYPY DEDYKOWANE (NewTypes z walidacją) ---

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MasterKeyB64(pub String);

impl Deref for MasterKeyB64 {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de> Deserialize<'de> for MasterKeyB64 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let decoded = BASE64
            .decode(&s)
            .map_err(|_| serde::de::Error::custom("master_key_b64 must be a valid Base64 string"))?;

        if decoded.len() != 32 {
            return Err(serde::de::Error::custom(
                "master_key_b64 after Base64 decoding must be exactly 32 bytes (256-bit key for AES-256-GCM)",
            ));
        }

        Ok(MasterKeyB64(s))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyTtlDays(pub u64);

impl Deref for KeyTtlDays {
    type Target = u64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de> Deserialize<'de> for KeyTtlDays {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val = u64::deserialize(deserializer)?;
        if !(1..=3650).contains(&val) {
            return Err(serde::de::Error::custom(
                "key_ttl_days must be between 1 and 3650 days",
            ));
        }
        Ok(KeyTtlDays(val))
    }
}

// --- GŁÓWNA STRUKTURA KONFIGURACJI KMS ---

#[derive(Debug, Deserialize, Clone)]
pub struct CryptoSettings {
    pub master_key_b64: MasterKeyB64,
    pub default_key_ttl_days: KeyTtlDays,
}