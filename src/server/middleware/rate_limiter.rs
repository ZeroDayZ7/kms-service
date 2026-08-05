use crate::infrastructure::redis::rate_limiter::RedisRateLimiter;
use axum::body::Body;
use governor::middleware::StateInformationMiddleware;
use std::sync::Arc;
use tower_governor::{
    GovernorLayer, governor::GovernorConfigBuilder, key_extractor::SmartIpKeyExtractor,
};
type AxumGovernorLayer = GovernorLayer<SmartIpKeyExtractor, StateInformationMiddleware, Body>;
use crate::config::Settings;

#[derive(Clone)]
pub struct RateLimitLayers {
    pub global: AxumGovernorLayer,
    pub health: AxumGovernorLayer,
    pub auth: AxumGovernorLayer,
    pub redis_limiter: Arc<RedisRateLimiter>,
}

impl RateLimitLayers {
    // Dodajemy redis_limiter do argumentów funkcji new
    pub fn new(settings: &Settings, limiter: Arc<RedisRateLimiter>) -> Self {
        let global_conf = GovernorConfigBuilder::default()
            .key_extractor(SmartIpKeyExtractor)
            .per_second(settings.rate_limit.global_per_second)
            .burst_size(settings.rate_limit.global_burst)
            .use_headers()
            .finish()
            .unwrap();

        let health_conf = GovernorConfigBuilder::default()
            .key_extractor(SmartIpKeyExtractor)
            .per_second(settings.rate_limit.health_per_second)
            .burst_size(settings.rate_limit.health_burst)
            .use_headers()
            .finish()
            .unwrap();

        let auth_conf = GovernorConfigBuilder::default()
            .key_extractor(SmartIpKeyExtractor)
            .per_second(settings.rate_limit.auth_per_second)
            .burst_size(settings.rate_limit.auth_burst)
            .use_headers()
            .finish()
            .unwrap();

        Self {
            global: GovernorLayer::new(Arc::new(global_conf)),
            health: GovernorLayer::new(Arc::new(health_conf)),
            auth: GovernorLayer::new(Arc::new(auth_conf)),
            redis_limiter: limiter,
        }
    }
}
