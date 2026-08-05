// src/server/state.rs
use crate::application::use_cases::{
    GenerateKeyPairUseCase, GetPrivateKeyUseCase, GetPublicKeyUseCase, RotateKeyUseCase,
    UnlockSecretUseCase,
};
use crate::config::Settings;
use crate::errors::AppResult;
use crate::infrastructure::crypto::kms_service::KmsCryptoService;
use crate::infrastructure::redis::client::RedisManager;
use crate::infrastructure::redis::rate_limiter::RedisRateLimiter;
use crate::infrastructure::serialization::JsonDecoder;
use crate::infrastructure::{MongoKeyRepository, MongoUserRepository, MongoVaultRepository};
use crate::services::user_service::UserService;

use mongodb::Database;
use std::sync::Arc;

pub type ConcreteUnlockSecretUseCase =
    UnlockSecretUseCase<MongoVaultRepository, KmsCryptoService, JsonDecoder>;
pub type ConcreteGenerateKeyPairUseCase = GenerateKeyPairUseCase<MongoKeyRepository>;
pub type ConcreteGetPublicKeyUseCase = GetPublicKeyUseCase<MongoKeyRepository>;
pub type ConcreteGetPrivateKeyUseCase = GetPrivateKeyUseCase<MongoKeyRepository>;
pub type ConcreteRotateKeyUseCase = RotateKeyUseCase<MongoKeyRepository>;

pub struct UseCases {
    pub unlock_secret: Arc<ConcreteUnlockSecretUseCase>,
    pub generate_key_pair: Arc<ConcreteGenerateKeyPairUseCase>,
    pub get_public_key: Arc<ConcreteGetPublicKeyUseCase>,
    pub get_private_key: Arc<ConcreteGetPrivateKeyUseCase>,
    pub rotate_key: Arc<ConcreteRotateKeyUseCase>,
}

#[derive(Clone)]
pub struct AppState {
    pub settings: Arc<Settings>,
    pub use_cases: Arc<UseCases>,
    pub redis_rate_limiter: Arc<RedisRateLimiter>,
    pub db: Database,
    pub redis_manager: Arc<RedisManager>,
    pub key_repo: Arc<MongoKeyRepository>,
}

impl AppState {
    pub async fn new(settings: Arc<Settings>) -> AppResult<Self> {
        let mongo_db = crate::infrastructure::database::init_mongo(&settings.database).await?;
        let redis_manager = Arc::new(RedisManager::new(&settings.redis).await?);

        let db_pool = Arc::new(mongo_db.clone());

        let vault_repo = Arc::new(MongoVaultRepository::new(Arc::clone(&db_pool)));
        let user_repo = Arc::new(MongoUserRepository::new(Arc::clone(&db_pool)));
        let key_repo = Arc::new(MongoKeyRepository::new(Arc::clone(&db_pool)));

        key_repo.ensure_indexes().await?;

        let crypto_service = Arc::new(KmsCryptoService::new(&settings.crypto)?);
        let decoder = Arc::new(JsonDecoder);

        let unlock_secret_use_case = Arc::new(UnlockSecretUseCase::new(
            vault_repo,
            crypto_service.clone(),
            decoder,
        ));

        let generate_key_pair_use_case = Arc::new(GenerateKeyPairUseCase::new(
            key_repo.clone(),
            crypto_service.clone(),
        ));
        let get_public_key_use_case = Arc::new(GetPublicKeyUseCase::new(key_repo.clone()));
        let get_private_key_use_case = Arc::new(GetPrivateKeyUseCase::new(
            key_repo.clone(),
            crypto_service.clone(),
        ));
        let rotate_key_use_case = Arc::new(RotateKeyUseCase::new(
            key_repo.clone(),
            crypto_service.clone(),
        ));

        let _user_service = Arc::new(UserService::new(user_repo));

        Ok(Self {
            settings,
            use_cases: Arc::new(UseCases {
                unlock_secret: unlock_secret_use_case,
                generate_key_pair: generate_key_pair_use_case,
                get_public_key: get_public_key_use_case,
                get_private_key: get_private_key_use_case,
                rotate_key: rotate_key_use_case,
            }),
            redis_rate_limiter: Arc::new(RedisRateLimiter::new(Arc::clone(&redis_manager)).await),
            db: mongo_db,
            redis_manager,
            key_repo,
        })
    }
}
