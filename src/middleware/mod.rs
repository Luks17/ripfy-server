pub mod auth;
pub mod cookies;
mod error;

/// The expected name for the auth-token in the request header
pub const AUTH_TOKEN: &str = "auth-token";
