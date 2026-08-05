// src/infrastructure/redis/keys.rs

use crate::domain::value_objects::api_route::ApiRoute;
use crate::domain::value_objects::client_ip::ClientIp;
use crate::domain::value_objects::session_token::SessionToken;
use crate::domain::value_objects::user_id::UserId;

pub struct RedisKey(String);

impl RedisKey {
    pub fn session(token: &SessionToken) -> Self {
        Self(format!("auth:session:{}", token.as_str()))
    }

    pub fn user_profile(id: &UserId) -> Self {
        Self(format!("user:profile:{}", id.to_string()))
    }

    pub fn rate_limit(route: ApiRoute, ip: &ClientIp) -> Self {
        Self(format!("rl:{}:{}", route.as_str(), ip.as_str()))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for RedisKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
