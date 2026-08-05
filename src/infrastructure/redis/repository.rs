// src/infrastructure/redis/repository.rs
use crate::domain::auth::repository::AuthRepository;
use crate::domain::value_objects::session_token::SessionToken;
use crate::domain::value_objects::session_ttl::SessionTtl;
use crate::domain::value_objects::user_id::UserId;
use crate::errors::AppResult;
use crate::infrastructure::redis::client::RedisManager;
use crate::infrastructure::redis::keys::RedisKey;
use std::sync::Arc;

pub struct RedisAuthRepository {
    redis: Arc<RedisManager>,
}

impl RedisAuthRepository {
    pub fn new(redis: Arc<RedisManager>) -> Self {
        Self { redis }
    }
}

impl AuthRepository for RedisAuthRepository {
    async fn store_session(
        &self,
        user_id: &UserId,
        token: &SessionToken,
        ttl: SessionTtl,
    ) -> AppResult<()> {
        let key = RedisKey::session(token);

        self.redis
            .set_ex(key.as_str(), &user_id.to_string(), ttl.as_secs())
            .await
    }

    async fn get_session(&self, token: &SessionToken) -> AppResult<Option<UserId>> {
        let key = RedisKey::session(token);

        let result: Option<String> = self.redis.get(key.as_str()).await?;

        result.map(|id_str| UserId::parse(&id_str)).transpose()
    }

    async fn delete_session(&self, token: &SessionToken) -> AppResult<()> {
        let key = RedisKey::session(token);
        self.redis.del(key.as_str()).await
    }
}
