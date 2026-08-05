// src/server/handlers/vault.rs
use crate::domain::vault::DecryptedCV;
use crate::errors::AppResult;
use crate::server::state::AppState;
use axum::{Json, extract::State};
use mongodb::bson::oid::ObjectId;
use serde::Deserialize;
use tracing::instrument;

#[derive(Deserialize)]
pub struct UnlockRequest {
    pub cv_id: ObjectId,
    pub access_key: String,
}

#[instrument(
    skip(state, payload),
    fields(cv_id = %payload.cv_id),
    err
)]
pub async fn unlock_cv(
    State(state): State<AppState>,
    Json(payload): Json<UnlockRequest>,
) -> AppResult<Json<DecryptedCV>> {
    let cv = state
        .use_cases
        .unlock_cv
        .execute(&payload.cv_id.to_hex(), &payload.access_key)
        .await?;

    Ok(Json(cv))
}
