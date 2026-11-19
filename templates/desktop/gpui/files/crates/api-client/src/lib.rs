//! Typed API client boundary.

#[derive(Debug, thiserror::Error)]
pub enum ApiClientError {
    #[error("API mode is not configured")]
    NotConfigured,
}
