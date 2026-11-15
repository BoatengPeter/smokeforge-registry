use super::{Job, QueueError};

/// Encodes and decodes queue job envelopes.
pub trait Codec: Send + Sync {
    fn encode(&self, job: &Job) -> Result<Vec<u8>, QueueError>;
    fn decode(&self, data: &[u8]) -> Result<Job, QueueError>;
}

/// Default JSON codec.
pub struct JsonCodec;

impl Codec for JsonCodec {
    fn encode(&self, job: &Job) -> Result<Vec<u8>, QueueError> {
        job.encode()
    }

    fn decode(&self, data: &[u8]) -> Result<Job, QueueError> {
        Job::decode(data)
    }
}
