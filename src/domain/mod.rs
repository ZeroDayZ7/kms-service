// Copyright 2026 ZeroDayZ7
// Licensed under the Apache License, Version 2.0
// See LICENSE file for details.

pub mod auth;
pub mod crypto;
pub mod ports;
pub mod rate_limiter;
pub mod user;
pub mod value_objects;
pub mod vault;

use self::user::User;
use self::vault::EncryptedCV;
use crate::errors::AppResult;
use async_trait::async_trait;

// Port dla Użytkownika
#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn find_by_email(&self, email: &str) -> AppResult<Option<User>>;
    async fn save(&self, user: User) -> AppResult<()>;
}

// Port dla Vault
#[async_trait]
pub trait VaultRepository: Send + Sync + 'static {
    async fn get_cv_by_id(&self, id: &str) -> AppResult<Option<EncryptedCV>>;
}