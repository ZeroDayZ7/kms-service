use crate::errors::app_error::AppError;
use rand::{RngExt, distr::Alphanumeric};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionToken(String);

impl SessionToken {
    pub fn new(value: String) -> Result<Self, AppError> {
        if value.len() < 32 || value.chars().any(|c| c.is_whitespace()) {
            return Err(AppError::ValidationError("Invalid session token".into()));
        }
        Ok(Self(value))
    }

    pub fn generate() -> Self {
        let token: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(64)
            .map(char::from)
            .collect();

        Self(token)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
