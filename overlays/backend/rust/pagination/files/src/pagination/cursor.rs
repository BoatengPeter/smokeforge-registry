use std::collections::HashMap;

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use hmac::{Hmac, KeyInit, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

use super::PaginationError;

const CURSOR_VERSION: u8 = 1;
const MIN_SECRET_LEN: usize = 32;

/// Signs and verifies opaque cursor values.
#[derive(Clone)]
pub struct Codec {
    key: Vec<u8>,
}

impl Codec {
    pub fn new(secret: &str) -> Result<Self, PaginationError> {
        if secret.len() < MIN_SECRET_LEN {
            return Err(PaginationError::Invalid(format!(
                "cursor secret must be at least {MIN_SECRET_LEN} bytes"
            )));
        }

        Ok(Self {
            key: secret.as_bytes().to_vec(),
        })
    }

    pub fn encode(&self, values: HashMap<String, String>) -> Result<String, PaginationError> {
        if values.is_empty() {
            return Err(PaginationError::Invalid(
                "cursor values must not be empty".to_owned(),
            ));
        }

        let payload = CursorPayload {
            version: CURSOR_VERSION,
            values,
        };
        let encoded = serde_json::to_vec(&payload)
            .map_err(|error| PaginationError::Invalid(error.to_string()))?;
        let body = URL_SAFE_NO_PAD.encode(encoded);
        let signature = self.sign(body.as_bytes())?;

        Ok(format!("{body}.{signature}"))
    }

    pub fn decode(&self, raw: &str) -> Result<HashMap<String, String>, PaginationError> {
        if raw.is_empty() || raw.len() > 4096 {
            return Err(PaginationError::InvalidCursor);
        }

        let (body, signature) = raw.split_once('.').ok_or(PaginationError::InvalidCursor)?;
        let expected = self.sign(body.as_bytes())?;

        if signature != expected {
            return Err(PaginationError::InvalidCursor);
        }

        let decoded = URL_SAFE_NO_PAD
            .decode(body)
            .map_err(|_| PaginationError::InvalidCursor)?;
        let payload: CursorPayload =
            serde_json::from_slice(&decoded).map_err(|_| PaginationError::InvalidCursor)?;

        if payload.version != CURSOR_VERSION || payload.values.is_empty() {
            return Err(PaginationError::InvalidCursor);
        }

        Ok(payload.values)
    }

    fn sign(&self, body: &[u8]) -> Result<String, PaginationError> {
        let mut mac = Hmac::<Sha256>::new_from_slice(&self.key)
            .map_err(|_| PaginationError::InvalidCursor)?;
        mac.update(body);

        Ok(URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes()))
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct CursorPayload {
    #[serde(rename = "v")]
    version: u8,
    #[serde(rename = "k")]
    values: HashMap<String, String>,
}
