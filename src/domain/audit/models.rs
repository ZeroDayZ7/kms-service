// src/domain/audit/models.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::domain::keys::models::{KeyAlgorithm, ServiceId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditAction {
    GetPrivateKey,
    GetPublicKey,
    GenerateKey,
    RotateKey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditStatus {
    Success,
    AccessDenied,
    NotFound,
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: uuid::Uuid,
    pub caller_service: ServiceId,
    pub target_service: ServiceId,
    pub action: AuditAction,
    pub algorithm: KeyAlgorithm,
    pub status: AuditStatus,
    pub reason: Option<String>,
    pub timestamp: DateTime<Utc>,
}