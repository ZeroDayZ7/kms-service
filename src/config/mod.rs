// src/config/mod.rs
use config::{Config, ConfigError, Environment, File};
use dotenvy::dotenv;

mod database;
mod log;
mod redis;
mod server;
mod settings;

pub mod acl;
pub mod cors;
pub mod crypto;
pub mod rate_limit;

pub use cors::HttpMethod;
pub use database::DatabaseConfig;
pub use log::LogConfig;
pub use log::LogLevel;
pub use redis::RedisConfig;
pub use settings::Settings;

pub fn load() -> Result<Settings, ConfigError> {
    let base_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("config")
        .join("settings.toml");

    load_from(base_path)
}

pub fn load_from<P: AsRef<std::path::Path>>(path: P) -> Result<Settings, ConfigError> {
    dotenv().ok();

    let settings_path = path.as_ref();
    let config_dir = settings_path
        .parent()
        .unwrap_or_else(|| std::path::Path::new("config"));
    let acl_path = config_dir.join("services_acl.toml");

    Config::builder()
        .add_source(File::from(settings_path).required(true))
        .add_source(File::from(acl_path).required(true))
        .add_source(Environment::default().separator("__"))
        .build()?
        .try_deserialize()
}
