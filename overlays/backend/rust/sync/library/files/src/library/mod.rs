//! Versioned Library resource and change feed for desktop offline sync.

pub mod routes;
pub mod service;

pub use routes::{document, router};
pub use service::LibraryService;
