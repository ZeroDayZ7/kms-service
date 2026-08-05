use crate::domain::vault::DecryptedSecret;
use crate::errors::AppResult;
use crate::server::state::AppState;
use axum::{Json, extract::State};
use mongodb::bson::oid::ObjectId;
use serde::Deserialize;
use tracing::instrument;

#[derive(Deserialize)]
pub struct UnlockRequest {
    pub secret_id: ObjectId,
    pub access_key: String,
}

#[instrument(
    skip(state, payload),
    fields(secret_id = %payload.secret_id),
    err
)]
pub async fn unlock_secret(
    State(state): State<AppState>,
    Json(payload): Json<UnlockRequest>,
) -> AppResult<Json<DecryptedSecret>> {
    let secret = state
        .use_cases
        .unlock_secret
        .execute(&payload.secret_id.to_hex())
        .await?;

    Ok(Json(secret))
}