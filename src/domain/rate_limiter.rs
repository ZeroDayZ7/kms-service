// src/domain/rate_limiter.rs
use crate::errors::AppResult;
use async_trait::async_trait;

#[async_trait]
pub trait RateLimiter: Send + Sync {
    async fn check(&self, ip: &str, path: &str, limit: u64) -> AppResult<RateLimitStatus>;
}

pub struct RateLimitStatus {
    pub allowed: bool,
    pub current: u64,
}
