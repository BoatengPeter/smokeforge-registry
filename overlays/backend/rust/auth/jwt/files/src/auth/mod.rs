//! JWT authentication.

mod password;
mod repository;
mod routes;
mod service;
mod token;
mod user;
// smokeforge:anchor auth-modules

pub use routes::document;
pub use routes::router;
pub use service::AuthService;
