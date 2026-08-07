use crate::domain::crypto::EncryptedPrivateKey;
use crate::errors::AppResult;
use crate::server::state::AppState;
use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct EncryptRequest {
    #[serde(with = "serde_bytes")]
    pub plaintext: Vec<u8>,
}

#[derive(Serialize)]
pub struct EncryptResponse {
    #[serde(with = "serde_bytes")]
    pub ciphertext: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub nonce: Vec<u8>,
    pub master_key_version: i32,
}

#[derive(Deserialize)]
pub struct DecryptRequest {
    #[serde(with = "serde_bytes")]
    pub ciphertext: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub nonce: Vec<u8>,
    pub master_key_version: i32,
}

#[derive(Serialize)]
pub struct DecryptResponse {
    #[serde(with = "serde_bytes")]
    pub plaintext: Vec<u8>,
}

pub async fn encrypt_handler(
    State(state): State<AppState>,
    Json(payload): Json<EncryptRequest>,
) -> AppResult<Json<EncryptResponse>> {
    let encrypted = state.use_cases.encrypt_data.execute(&payload.plaintext)?;

    Ok(Json(EncryptResponse {
        ciphertext: encrypted.ciphertext,
        nonce: encrypted.nonce,
        master_key_version: encrypted.master_key_version,
    }))
}

pub async fn decrypt_handler(
    State(state): State<AppState>,
    Json(payload): Json<DecryptRequest>,
) -> AppResult<Json<DecryptResponse>> {
    let payload_struct = EncryptedPrivateKey {
        ciphertext: payload.ciphertext,
        nonce: payload.nonce,
        master_key_version: payload.master_key_version,
    };

    let decrypted = state.use_cases.decrypt_data.execute(&payload_struct)?;

    Ok(Json(DecryptResponse {
        plaintext: decrypted,
    }))
}
