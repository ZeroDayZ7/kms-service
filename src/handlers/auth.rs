// Copyright 2026 ZeroDayZ7
// Licensed under the Apache License, Version 2.0
// See LICENSE file for details.

use crate::errors::{AppError, AppResult};
use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct LoginResponse {
    pub message: String,
}

pub async fn login() -> AppResult<Json<LoginResponse>> {
    let fail = true;

    if fail {
        return Err(AppError::Unauthorized);
    }

    Ok(Json(LoginResponse {
        message: "Login successful".to_string(),
    }))
}
