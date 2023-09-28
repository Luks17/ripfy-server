use super::{
    error::{Error, Result},
    AUTH_TOKEN,
};
use crate::context::Ctx;
use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, Request},
    middleware::Next,
    response::Response,
};
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};

/// Middleware for extracting token cookie from request header and returning a context
pub async fn ctx_resolver<B>(
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    tracing::debug!("MIDDLEWARE - CTX_RESOLVER");

    // extracts auth_token from cookies and parses it on the parse_token function
    let (user_id, _expiration, _signature) = cookies
        .get(AUTH_TOKEN)
        .map(|c| c.value().to_string())
        .ok_or(Error::AuthTokenRequired)
        .and_then(parse_token)?;

    // TODO: token validation
    // If the client sends an invalid cookie, we want to remove it
    if false {
        cookies.remove(Cookie::named(AUTH_TOKEN))
    }

    // Store the ctx_result in the request extension.
    req.extensions_mut().insert(Ctx::new(user_id));

    Ok(next.run(req).await)
}

// Implements FromRequestParts and not FromRequest because it does not need the request body
#[async_trait]
impl<S> FromRequestParts<S> for Ctx
where
    S: Send + Sync,
{
    type Rejection = Error;

    /// Returns a result of a new instance of Ctx
    /// Gets it from the request parts extensions after the ctx_resolver middleware stores it there
    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self> {
        tracing::debug!("MIDDLEWARE - EXTRACTOR - CTX");

        parts
            .extensions
            .get::<Result<Ctx>>()
            .ok_or(Error::CtxNotInRequestExtensions)?
            .clone()
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
