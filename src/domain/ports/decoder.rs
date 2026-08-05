// src/domain/ports/decoder.rs
use crate::errors::AppResult;

pub trait Decoder<T>: Send + Sync {
    fn decode(&self, bytes: &[u8]) -> AppResult<T>;
}
