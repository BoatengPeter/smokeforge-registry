//! JWT authentication.

mod password;
mod repository;
mod routes;
mod service;
mod token;
mod user;

pub use routes::router;
pub use service::AuthService;
