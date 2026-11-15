use std::io::{Read, Write};

use flate2::{Compression, read::GzDecoder, write::GzEncoder};

use super::{
    Job, QueueError,
    codec::{Codec, JsonCodec},
};

/// Gzip codec that only compresses larger job envelopes.
pub struct GzipCodec {
    inner: JsonCodec,
    min_bytes: usize,
}

impl GzipCodec {
    pub fn new(min_bytes: usize) -> Self {
        Self {
            inner: JsonCodec,
            min_bytes,
        }
    }
}

impl Codec for GzipCodec {
    fn encode(&self, job: &Job) -> Result<Vec<u8>, QueueError> {
        let data = self.inner.encode(job)?;
        if data.len() < self.min_bytes {
            return Ok(data);
        }

        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder
            .write_all(&data)
            .map_err(|error| QueueError::Codec(error.to_string()))?;
        encoder
            .finish()
            .map_err(|error| QueueError::Codec(error.to_string()))
    }

    fn decode(&self, data: &[u8]) -> Result<Job, QueueError> {
        if data.starts_with(&[0x1f, 0x8b]) {
            let mut decoder = GzDecoder::new(data);
            let mut decoded = Vec::new();
            decoder
                .read_to_end(&mut decoded)
                .map_err(|error| QueueError::Codec(error.to_string()))?;

            return self.inner.decode(&decoded);
        }

        self.inner.decode(data)
    }
}
