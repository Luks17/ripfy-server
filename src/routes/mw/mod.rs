pub mod auth;
pub mod cookies;
use super::error;

/// Format: (user_id, expiration, signature)
struct Token(u64, String, String);

/// The expected name for the auth-token in the request header
pub const AUTH_TOKEN: &str = "auth-token";
