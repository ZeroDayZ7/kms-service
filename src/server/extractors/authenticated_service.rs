use axum::{extract::FromRequestParts, http::request::Parts};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use subtle::ConstantTimeEq;

use crate::{domain::keys::models::ServiceId, errors::AppError, server::state::AppState};

type HmacSha256 = Hmac<Sha256>;

pub struct AuthenticatedService(pub ServiceId);

impl FromRequestParts<AppState> for AuthenticatedService {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let service_name = parts
            .headers
            .get("X-Service-Name")
            .and_then(|v| v.to_str().ok())
            .ok_or(AppError::Unauthorized)?;

        let timestamp = parts
            .headers
            .get("X-Timestamp")
            .and_then(|v| v.to_str().ok())
            .ok_or(AppError::Unauthorized)?;

        let signature_hex = parts
            .headers
            .get("X-HMAC-Signature")
            .and_then(|v| v.to_str().ok())
            .ok_or(AppError::Unauthorized)?;

        let service_cfg = state
            .settings
            .acl
            .services
            .get(service_name)
            .ok_or(AppError::Unauthorized)?;

        let method = parts.method.as_str();
        let path = parts.uri.path();
        let payload_to_sign = format!("{method}:{path}:{timestamp}");

        let mut mac = HmacSha256::new_from_slice(service_cfg.secret.as_bytes())
            .map_err(|_| AppError::Internal("Błąd inicjalizacji HMAC".into()))?;
        mac.update(payload_to_sign.as_bytes());

        let expected_signature = hex::encode(mac.finalize().into_bytes());

        if signature_hex
            .as_bytes()
            .ct_eq(expected_signature.as_bytes())
            .unwrap_u8()
            != 1
        {
            return Err(AppError::Unauthorized);
        }

        Ok(AuthenticatedService(ServiceId(service_name.to_string())))
    }
}
