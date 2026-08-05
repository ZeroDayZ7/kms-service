// src/handlers/mod.rs
pub mod auth;
pub mod health;
pub mod keys;
pub mod vault;

pub use auth::*;
pub use health::*;
pub use keys::*;
pub use vault::*;
