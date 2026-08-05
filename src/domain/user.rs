// src/domain/user.rs
use serde::{Deserialize, Serialize};

use crate::domain::value_objects::email::Email;
use crate::domain::value_objects::user_id::UserId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Option<UserId>,
    pub email: Email,
    pub password_hash: String,
}
