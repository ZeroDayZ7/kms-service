// src/main.rs
use kms_service::bootstrap::bootstrap_keys;
use kms_service::config;
use kms_service::server::{self, state::AppState};

use anyhow::Context;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("❌ KRYTYCZNY BŁĄD: {:#}", e);
        error!(error = ?e, "❌ Fatal application error");
        std::process::exit(1);
    }
}

async fn run() -> anyhow::Result<()> {
    // -------------------------
    // 1. CONFIG
    // -------------------------
    let settings = config::load().context("Failed to load configuration")?;
    let settings = Arc::new(settings);

    // -------------------------
    // 2. LOGGING
    // -------------------------
    server::logger::init_logging(settings.log.level);

    info!("⚙️ Configuration loaded");

    // -------------------------
    // 3. BUILD STATE (fail-fast)
    // -------------------------
    let state = AppState::new(settings.clone())
        .await
        .context("Krytyczny błąd inicjalizacji AppState")?;

    info!("🧠 Application state initialized");

    // -------------------------
    // 4. BOOTSTRAP KEYS (Inicjalizacja kluczy na podstawie ACL)
    // -------------------------
    bootstrap_keys(
        &settings.acl, // <-- POPRAWIONO: przekazujemy referencję do settings.acl
        state.key_repo.clone(),
        state.crypto_service.clone(),
    )
    .await
    .context("Krytyczny błąd bootstrapu kluczy KMS")?;

    // -------------------------
    // 5. ADDRESS
    // -------------------------
    let addr: SocketAddr = format!("{}:{}", settings.server.host, settings.server.port)
        .parse()
        .context("Invalid server address")?;

    // -------------------------
    // 6. ROUTER
    // -------------------------
    let app = server::router(state);

    info!("🚀 Server starting on {}", addr);

    // -------------------------
    // 7. SERVER LIFECYCLE
    // -------------------------
    server::http::serve(app, addr, settings.server.shutdown_timeout)
        .await
        .context("HTTP server crashed")?;

    info!("✅ Server shutdown complete");

    Ok(())
}
