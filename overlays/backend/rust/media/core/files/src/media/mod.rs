//! Authenticated private media metadata and presigned storage access.

mod repository;
mod routes;
mod s3;
mod service;

pub use routes::router;
pub use service::MediaService;
