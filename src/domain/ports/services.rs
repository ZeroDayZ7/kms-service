// src/domain/ports/services.rs
use crate::domain::user::User;
use crate::domain::vault::DecryptedCV;
use crate::errors::AppResult;
use async_trait::async_trait;

#[async_trait]
pub trait VaultServicePort: Send + Sync {
    async fn unlock_cv(&self, id: &str, key: &str) -> AppResult<DecryptedCV>;
}

#[async_trait]
pub trait UserServicePort: Send + Sync {
    async fn get_user_by_email(&self, email: &str) -> AppResult<User>;
    async fn register_user(&self, user: User) -> AppResult<()>;
}
