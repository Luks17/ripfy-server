use super::{
    error::{Error, Result},
    AUTH_TOKEN,
};
use crate::context::Ctx;
use async_trait::async_trait;
use axum::{extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use lazy_regex::regex_captures;
use tower_cookies::Cookies;

/// Extracts cookie token from response header
/// Implements FromRequestParts and not FromRequest because it does not need the request body
#[async_trait]
impl<S> FromRequestParts<S> for Ctx
where
    S: Send + Sync,
{
    type Rejection = Error;

    /// Returns a result of a new instance of Ctx
    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self> {
        tracing::debug!("MIDDLEWARE - EXTRACTOR - CTX");

        let cookies = parts.extract::<Cookies>().await.unwrap();

        let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

        // Runs parse_token() on the auth_toke
        let (user_id, _expiration, _sign) = auth_token
            .ok_or(Error::AuthTokenRequired)
            .and_then(parse_token)?;

        Ok(Ctx::new(user_id))
    }
}

/// Parse a token of format `user-[user-id].[expiration].[signature]`
///
/// Returns (user_id, expiration, signature)
/// Returns Error if token format is invalid
fn parse_token(token: String) -> Result<(u64, String, String)> {
    // gets the whole match for the first parameter and the others in the next ones
    let (_, user_id, expiration, sign) = regex_captures!(
        r#"^user-(\d+)\.(.+)\.(.+)"#, // a literal regex
        &token
    )
    .ok_or(Error::AuthTokenWrongFormat)?;

    // if cannot parse the user_id to u64, then the authToken format must be invalid
    let user_id: u64 = user_id.parse().map_err(|_| Error::AuthTokenWrongFormat)?;

    Ok((user_id, expiration.to_string(), sign.to_string()))
}
