pub mod ctx;
use super::{error, gen_and_set_token_cookie};

/// The expected name for the auth-token in the request header
pub const AUTH_TOKEN: &str = "auth-token";
