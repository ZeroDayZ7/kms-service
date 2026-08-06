// src/domain/audit/repository.rs
use async_trait::async_trait;
use crate::{domain::audit::models::AuditLog, errors::AppResult};

#[async_trait]
pub trait AuditRepository: Send + Sync {
    async fn record(&self, log: AuditLog) -> AppResult<()>;
}