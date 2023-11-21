pub mod auth;
mod error;
pub mod mw;
pub mod song;

use crate::crypt::token::Token;
use error::Result;
use mw::AUTH_TOKEN;
use serde::Serialize;
use tower_cookies::{Cookie, Cookies};

#[derive(Debug, Serialize)]
pub struct ModelResponse<T> {
    pub data: T,
}

/// Receives a cookie jar ref and an identifier
/// Generates an access token for said identifier and stores it on the jar
/// Returns Err if token generation fails
async fn gen_and_set_token_cookie(cookies: &Cookies, identifier: &str) -> Result<()> {
    let token = Token::new_access_token(identifier)?;

    // sets access token to cookies
    let mut cookie = Cookie::new(AUTH_TOKEN, token.to_string());
    cookie.set_http_only(true);
    cookie.set_path("/");
    cookies.add(cookie);

    Ok(())
}

async fn remove_token_cookie(cookies: &Cookies) {
    let mut cookie = Cookie::named(AUTH_TOKEN);
    cookie.set_path("/");

    cookies.remove(cookie);
}
