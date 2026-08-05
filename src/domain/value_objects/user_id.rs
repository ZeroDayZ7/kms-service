// src/domain/value_objects/user_id.rs
use crate::errors::app_error::AppError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(String);

impl UserId {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn parse(value: &str) -> Result<Self, AppError> {
        if value.trim().is_empty() {
            return Err(AppError::ValidationError("UserId cannot be empty".into()));
        }
        Ok(Self(value.to_string()))
    }

    pub fn as_inner(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}