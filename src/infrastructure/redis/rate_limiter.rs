use crate::errors::AppResult;
use crate::infrastructure::redis::client::RedisManager;
use crate::infrastructure::redis::keys::RedisKey;
use fred::interfaces::LuaInterface;
use std::sync::Arc;
use tracing::warn;

const LUA_SCRIPT: &str = include_str!("../scripts/redis_rate_limit.lua");

pub struct RateLimitResult {
    pub allowed: bool,
    pub current: u64,
}

#[derive(Clone)]
pub struct RedisRateLimiter {
    redis: Arc<RedisManager>,
    script_hash: String,
}

impl RedisRateLimiter {
    pub async fn new(redis: Arc<RedisManager>) -> Self {
        let client = redis.client();

        let script_hash = match client.script_load(LUA_SCRIPT).await {
            Ok(hash) => hash,
            Err(e) => {
                warn!(
                    "⚠️ Nie udało się załadować skryptu: {}. Fallback do EVAL.",
                    e
                );
                String::new()
            }
        };

        Self { redis, script_hash }
    }

    pub async fn check(
        &self,
        key: &RedisKey,
        limit: u64,
        window_sec: u64,
    ) -> AppResult<RateLimitResult> {
        let client = self.redis.client();
        let key_str = key.as_str();
        let args = vec![window_sec.to_string()];

        let result: i64 = if !self.script_hash.is_empty() {
            match client
                .evalsha::<i64, _, _, _>(&self.script_hash, vec![key_str], args.clone())
                .await
            {
                Ok(res) => res,
                Err(_) => self.fallback_eval(key_str, args).await?,
            }
        } else {
            self.fallback_eval(key_str, args).await?
        };

        Ok(RateLimitResult {
            allowed: result as u64 <= limit,
            current: result as u64,
        })
    }

    async fn fallback_eval(&self, key: &str, args: Vec<String>) -> AppResult<i64> {
        // To zadziała, bo fred::Error -> AppError::RedisError
        let res = self
            .redis
            .client()
            .eval::<i64, _, _, _>(LUA_SCRIPT, vec![key], args)
            .await?;

        Ok(res)
    }
}
