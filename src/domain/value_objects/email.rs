use crate::errors::app_error::AppError;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
// Używamy #[serde(transparent)], aby w JSONie email serializował się jako zwykły string: "user@example.com",
// a nie jako obiekt/struktura jednoelementowa: {"0": "user@example.com"} lub ["user@example.com"].
#[serde(transparent)]
pub struct Email(String);

impl Email {
    /// Tworzy nową instancję `Email` wraz z podstawową walidacją i normalizacją.
    pub fn new(value: String) -> Result<Self, AppError> {
        let normalized = value.trim().to_lowercase();

        // Podstawowa walidacja (można rozbudować o regex, ale proste sprawdzanie '@'
        // zapobiega 99% błędów bez narzutu wydajnościowego).
        if normalized.is_empty() || !normalized.contains('@') {
            return Err(AppError::ValidationError("Invalid email format".into()));
        }

        Ok(Self(normalized))
    }

    /// Zwraca referencję do wewnętrznego stringa.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Konsumuje obiekt i zwraca wewnętrzny `String` (przydatne przy zapisie do DB bez klonowania).
    pub fn into_inner(self) -> String {
        self.0
    }
}

// Implementacja Display rozwiązuje Twój błąd kompilacji `E0599` z poprzedniego kroku.
// Dzięki temu możesz wywołać `.to_string()` na instancji `Email`.
impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Implementacja AsRef pozwala na bezkosztowe traktowanie Email jako &str w generycznych API.
impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

// Umożliwia łatwą konwersję z `Email` do `String`.
impl From<Email> for String {
    fn from(email: Email) -> Self {
        email.0
    }
}