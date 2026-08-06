// src/bootstrap.rs
use std::sync::Arc;
use tracing::{info, warn};

use crate::{
    config::acl::AclSettings,
    domain::{
        crypto::KmsCryptoService,
        keys::{
            models::{KeyAlgorithm, KeyPairEntity, KeyPurpose},
            repository::KeyRepository,
        },
    },
    errors::AppResult,
};

pub async fn bootstrap_keys<R>(
    acl_settings: &AclSettings,
    key_repo: Arc<R>,
    crypto_service: Arc<dyn KmsCryptoService + Send + Sync>,
) -> AppResult<()>
where
    R: KeyRepository,
{
    info!("Rozpoczynanie weryfikacji i bootstrapu kluczy z konfiguracji ACL...");

    for service_cfg in acl_settings.services.values() {
        for rule in &service_cfg.allowed_access {
            let target_service = rule.target_service.clone();
            let algorithm = rule.algorithm;

            // Sprawdzamy, czy aktywny klucz już istnieje w MongoDB
            let existing_key = key_repo.get_active_key(&target_service, algorithm).await?;

            if existing_key.is_none() {
                warn!(
                    service = %target_service.0,
                    alg = ?algorithm,
                    "Brak aktywnego klucza w MongoDB. Generowanie nowego klucza..."
                );

                // 1. Generowanie pary kluczy i wyznaczenie przeznaczenia na podstawie algorytmu
                let (generated_key, purpose) = match algorithm {
                    KeyAlgorithm::Ed25519 => (
                        crypto_service.generate_ed25519_keypair()?,
                        KeyPurpose::Signing,
                    ),
                    KeyAlgorithm::X25519 => (
                        crypto_service.generate_x25519_keypair()?,
                        KeyPurpose::Encryption,
                    ),
                };

                // 2. Szyfrowanie klucza prywatnego Master Keyem
                let encrypted_private_key =
                    crypto_service.encrypt_private_key(&generated_key.private_key_bytes)?;

                // 3. Utworzenie encji domenowej KeyPairEntity
                let new_key = KeyPairEntity {
                    id: uuid::Uuid::now_v7(),
                    service_id: target_service.clone(),
                    algorithm,
                    purpose,
                    public_key_pem: generated_key.public_key_pem.clone(), // Klonowanie zapobiega ruchowi ze struktury z `Drop`
                    encrypted_private_key,
                    version: 1,
                    is_active: true,
                    created_at: chrono::Utc::now(),
                    expires_at: None,
                };

                // 4. Zapis w bazie danych
                key_repo.save_key(&new_key).await?;
                info!(
                    service = %target_service.0,
                    alg = ?algorithm,
                    "Pomyślnie utworzono i zaszyfrowano klucz w MongoDB."
                );
            }
        }
    }

    info!("Bootstrap kluczy zakończony sukcesem.");
    Ok(())
}
