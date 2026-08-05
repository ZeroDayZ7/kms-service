// src/infrastructure/serialization/json_decoder.rs
use crate::domain::ports::decoder::Decoder;
use crate::errors::{AppError, AppResult};
use serde::de::DeserializeOwned;

pub struct JsonDecoder;

impl<T: DeserializeOwned> Decoder<T> for JsonDecoder {
    fn decode(&self, bytes: &[u8]) -> AppResult<T> {
        serde_json::from_slice(bytes).map_err(AppError::from)
    }
}
