// src/domain/auth/repository.rs
use crate::domain::value_objects::session_token::SessionToken;
use crate::domain::value_objects::session_ttl::SessionTtl;
use crate::domain::value_objects::user_id::UserId;
use crate::errors::AppResult;

#[allow(async_fn_in_trait)]
pub trait AuthRepository: Send + Sync {
    async fn store_session(
        &self,
        user_id: &UserId,
        token: &SessionToken,
        ttl: SessionTtl,
    ) -> AppResult<()>;

    async fn get_session(&self, token: &SessionToken) -> AppResult<Option<UserId>>;

    async fn delete_session(&self, token: &SessionToken) -> AppResult<()>;
}
