pub mod auth;
mod error;
pub mod mw;
pub mod payloads;
pub mod playlist;
pub mod song;
pub mod stream;

use crate::crypt::token::Token;
use entity::playlist::Model as Playlist;
use entity::song::Model as Song;
use entity::user::Model as User;
use error::Result;
use mw::AUTH_TOKEN;
use serde::{Deserialize, Serialize};
use tower_cookies::{Cookie, Cookies};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[aliases(
    ResponseModelUser = ResponseModel<User>,
    ResponseModelSong = ResponseModel<Song>,
    ResponseModelPlaylist = ResponseModel<Playlist>
)]
pub struct ResponseModel<T> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
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
