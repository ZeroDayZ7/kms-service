use config::{Config, Environment, File};
use dotenvy::dotenv;

mod database;
mod log;
mod redis;
mod server;
mod settings;

pub mod cors;
pub mod crypto;
pub mod rate_limit;

pub use cors::HttpMethod;
pub use database::DatabaseConfig;
pub use log::LogConfig;
pub use log::LogLevel;
pub use redis::RedisConfig;
pub use settings::Settings;

pub fn load() -> Result<Settings, config::ConfigError> {
    let base_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("config")
        .join("settings.toml");

    load_from(base_path)
}

pub fn load_from<P: AsRef<std::path::Path>>(path: P) -> Result<Settings, config::ConfigError> {
    dotenv().ok();

    Config::builder()
        .add_source(File::from(path.as_ref().to_path_buf()).required(true))
        .add_source(Environment::default().separator("__"))
        .build()?
        .try_deserialize()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config_file() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("config")
            .join("settings.toml");

        // To wypisze ścieżkę w konsoli, jeśli test zawiedzie (lub z flagą --nocapture)
        println!("Szukam pliku w: {:?}", path.display());
        println!("Czy plik istnieje fizycznie? {}", path.exists());

        let result = load_from(&path);

        assert!(
            result.is_ok(),
            "Nie udało się załadować konfiguracji z ścieżki: {:?}. Błąd: {:?}",
            path,
            result.err()
        );
    }
}
