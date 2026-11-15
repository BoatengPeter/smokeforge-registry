//! Authenticated private media metadata and presigned storage access.

mod imgproxy;
mod repository;
mod routes;
mod s3;
mod service;
mod transform;

pub use routes::router;
pub use service::MediaService;
pub use transform::Transform;
