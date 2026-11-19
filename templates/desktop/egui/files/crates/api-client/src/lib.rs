//! API contract client. The generated Progenitor client will be added with the
//! fullstack API feature; standalone mode has no network dependency.

#[derive(Debug, thiserror::Error)]
pub enum ApiClientError {
    #[error("API mode is not configured")]
    NotConfigured,
}
