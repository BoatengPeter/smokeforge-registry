use std::sync::Arc;

use async_trait::async_trait;

use crate::error::AppError;

/// Dependency readiness check used by `/readyz`.
#[async_trait]
pub trait Readiness: Send + Sync {
    async fn ready(&self) -> Result<(), AppError>;
}

/// Readiness check for a base app with no external dependencies.
pub struct AlwaysReady;

#[async_trait]
impl Readiness for AlwaysReady {
    async fn ready(&self) -> Result<(), AppError> {
        Ok(())
    }
}

/// Shared application state. Clone is cheap.
#[derive(Clone)]
pub struct AppState {
    service_name: String,
    readiness: Arc<dyn Readiness>,
}

impl AppState {
    pub fn new(service_name: impl Into<String>) -> Self {
        Self {
            service_name: service_name.into(),
            readiness: Arc::new(AlwaysReady),
        }
    }

    pub fn with_readiness(self, readiness: Arc<dyn Readiness>) -> Self {
        Self {
            service_name: self.service_name,
            readiness,
        }
    }

    pub fn service_name(&self) -> &str {
        &self.service_name
    }

    pub async fn ready(&self) -> Result<(), AppError> {
        self.readiness.ready().await
    }
}
