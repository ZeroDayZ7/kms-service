// Main entry point for the HTTP server application.
use kms_service::config;
use kms_service::server::{self, state::AppState};

use anyhow::Context;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        // Użyj {:#}, aby rozwinąć cały łańcuch błędów (wyświetla `caused by: ...`)
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

    let _guards = server::logger::init_logging(settings.log.level);

    info!("⚙️ Configuration loaded");

    // -------------------------
    // 3. BUILD STATE (fail-fast)
    // -------------------------
    let state = AppState::new(settings.clone())
        .await
        .context("Krytyczny błąd inicjalizacji AppState")?;

    info!("🧠 Application state initialized");

    // -------------------------
    // 4. ADDRESS
    // -------------------------
    let addr: SocketAddr = format!("{}:{}", settings.server.host, settings.server.port)
        .parse()
        .context("Invalid server address")?;

    // -------------------------
    // 5. ROUTER
    // -------------------------
    let app = server::router(state);

    info!("🚀 Server starting on {}", addr);

    // -------------------------
    // 6. SERVER LIFECYCLE
    // -------------------------
    server::http::serve(app, addr, settings.server.shutdown_timeout)
        .await
        .context("HTTP server crashed")?;

    info!("✅ Server shutdown complete");

    Ok(())
}
