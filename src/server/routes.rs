// src/server/routes.rs
use crate::handlers::{auth, health, keys, vault};
use crate::server::middleware::{self, RateLimitLayers};
use crate::server::state::AppState;

use axum::{
    Router,
    routing::{get, post},
};

pub fn router(state: AppState) -> Router {
    let cors = middleware::create_cors_layer(&state.settings);
    let security = middleware::create_security_headers_layer().into_inner();

    let rate_limits = RateLimitLayers::new(&state.settings, state.redis_rate_limiter.clone());

    let redis_mw = axum::middleware::from_fn_with_state(
        state.clone(),
        middleware::redis_rate_limit_middleware,
    );

    Router::new()
        // Standardowe endpointy
        .route(
            "/health",
            get(health::health).layer(rate_limits.health.clone()),
        )
        .route(
            "/auth/login",
            post(auth::login).layer(rate_limits.auth.clone()),
        )
        .route(
            "/vault/unlock",
            post(vault::unlock_secret).layer(rate_limits.auth.clone()),
        )
        .route(
            "/api/v1/keys/generate",
            post(keys::generate_key_handler).layer(rate_limits.auth.clone()),
        )
        .route(
            "/api/v1/keys/public/:service_id/:algorithm",
            get(keys::get_public_key_handler).layer(rate_limits.health.clone()),
        )
        .route(
            "/api/v1/keys/rotate",
            post(keys::rotate_key_handler).layer(rate_limits.auth.clone()),
        )
        .route(
            "/api/v1/keys/private",
            post(keys::get_private_key_handler).layer(rate_limits.auth.clone()),
        )
        // Middleware globalne
        .route_layer(rate_limits.global.clone())
        .layer(redis_mw)
        .layer(security)
        .layer(cors)
        .layer(middleware::http_trace_layer())
        .with_state(state)
}
