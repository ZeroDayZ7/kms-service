// src/bootstrap.rs
use std::sync::Arc;
use tracing::{info, warn};

use crate::{
    config::acl::AclSettings,
    domain::{
        crypto::KmsCryptoService,
        keys::{
            models::{KeyAlgorithm, KeyPurpose, ServiceId},
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
            let target_service = &rule.target_service;
            let algorithm = rule.algorithm;

            // Sprawdzamy, czy aktywny klucz już istnieje w MongoDB
            let existing_key = key_repo.get_active_key(target_service, algorithm).await?;

            if existing_key.is_none() {
                warn!(
                    service = %target_service.0,
                    alg = ?algorithm,
                    "Brak aktywnego klucza w MongoDB. Generowanie nowego klucza..."
                );

                // 1. Generowanie nowej pary kluczy / klucza symetrycznego
                let generated_key = crypto_service.generate_key_pair(algorithm)?;

                // 2. Szyfrowanie klucza prywatnego Master Keyem z KMS (np. AES-256-GCM z ENV)
                let encrypted_private_key =
                    crypto_service.encrypt_private_key(&generated_key.private_key_bytes)?;

                // 3. Utworzenie nowej encji domenowej
                let new_key = crate::domain::keys::models::KeyEntity {
                    id: uuid::Uuid::new_v4(),
                    service_id: target_service.clone(),
                    algorithm,
                    purpose: KeyPurpose::Signing,
                    public_key_pem: generated_key.public_key_pem,
                    encrypted_private_key,
                    version: 1,
                    is_active: true,
                    created_at: chrono::Utc::now(),
                };

                // 4. Zapis w bazie MongoDB
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