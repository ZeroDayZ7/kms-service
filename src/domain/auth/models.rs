// src/domain/auth/models.rs
use serde::{Deserialize, Serialize};

// Importy centralnych Value Objects
use crate::domain::value_objects::email::Email;
use crate::domain::value_objects::session_token::SessionToken;
use crate::domain::value_objects::session_ttl::SessionTtl;
use crate::domain::value_objects::user_id::UserId;

/// Payload dla próby logowania
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginPayload {
    pub email: Email,
    pub password: String,
}

/// Dane rejestracji nowego użytkownika
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterPayload {
    pub email: Email,
    pub password: String,
    pub confirm_password: String,
}

/// Pełna odpowiedź po poprawnym uwierzytelnieniu
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub user_id: UserId,
    pub token: SessionToken,
    pub expires_in: SessionTtl,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_id_from_value_objects() {
        let hex = "507f1f77bcf86cd799439011";
        let res = UserId::parse(hex);
        assert!(res.is_ok());
        assert_eq!(res.unwrap().to_hex(), hex);
    }

    #[test]
    fn test_email_validation_in_auth() {
        let valid_email = Email::new("test@zeroday.pl".to_string());
        assert!(valid_email.is_ok());

        let invalid_email = Email::new("wrong-email".to_string());
        assert!(invalid_email.is_err());
    }

    #[test]
    fn test_session_token_generation_logic() {
        // Testujemy czy generator z value_objects produkuje poprawną długość
        let token = SessionToken::generate();
        assert_eq!(token.as_str().len(), 64);

        // Testujemy walidację manualną
        let manual_token = SessionToken::new("a".repeat(32));
        assert!(manual_token.is_ok());
    }

    #[test]
    fn test_session_ttl_mapping() {
        let ttl = SessionTtl::from_secs(3600);
        assert_eq!(ttl.as_secs(), 3600);
    }
}
