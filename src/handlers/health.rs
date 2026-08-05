use crate::server::state::AppState;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use mongodb::bson::doc;
use serde::Serialize;
use std::time::Duration;
use tokio::time::timeout;

#[derive(Serialize)]
pub struct HealthResponse {
    status: &'static str,
    database: &'static str,
    redis: &'static str,
}

pub async fn health(State(state): State<AppState>) -> impl IntoResponse {
    let check_timeout = Duration::from_secs(2);

    let (db_res, redis_res) = tokio::join!(
        timeout(check_timeout, state.db.run_command(doc! {"ping": 1})),
        timeout(check_timeout, state.redis_manager.ping())
    );

    let db_status = match db_res {
        Ok(Ok(_)) => "ok",
        _ => "error",
    };

    let redis_status = match redis_res {
        Ok(Ok(_)) => "ok",
        _ => "error",
    };

    let is_ok = db_status == "ok" && redis_status == "ok";

    let status_code = if is_ok {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    (
        status_code,
        Json(HealthResponse {
            status: if is_ok { "ok" } else { "degraded" },
            database: db_status,
            redis: redis_status,
        }),
    )
}
