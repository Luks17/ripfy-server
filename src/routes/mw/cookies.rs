use super::{
    error::{Error, Result},
    Token, AUTH_TOKEN,
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
    // if the token exists and the parse is successful, the token is then validated
    // if the token is valid, returns ctx
    let ctx = cookies
        .get(AUTH_TOKEN)
        .map(|c| c.value().to_string())
        .ok_or(Error::NoAuthToken)
        .and_then(parse_token)
        .and_then(validate_token);

    if let Err(ref e) = ctx {
        // If the client sends an invalid cookie, we want to remove it
        if !matches!(*e, Error::NoAuthToken) {
            tracing::debug!("MIDDLEWARE - CTX_RESOLVER - REMOVING INVALID COOKIE FROM HEADER");
            cookies.remove(Cookie::named(AUTH_TOKEN));
        }
    }

    // Store the ctx_result in the request extension.
    req.extensions_mut().insert(ctx);

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

/// Parse a raw token of format `user-[user-id].[expiration].[signature]`
///
/// Returns (user_id, expiration, signature)
/// Returns Error if token format is invalid
fn parse_token(raw_token: String) -> Result<Token> {
    // gets the whole match for the first parameter and the others in the next ones
    let (_, user_id, expiration, sign) = regex_captures!(
        r#"^user-(\d+)\.(.+)\.(.+)"#, // a literal regex
        &raw_token
    )
    .ok_or(Error::AuthTokenWrongFormat)?;

    // if cannot parse the user_id to u64, then the authToken format must be invalid
    let user_id: u64 = user_id.parse().map_err(|_| Error::AuthTokenWrongFormat)?;

    Ok(Token(user_id, expiration.to_string(), sign.to_string()))
}

fn validate_token(Token(user_id, _expiration, _signature): Token) -> Result<Ctx> {
    // TODO: token validation

    Ok(Ctx::new(user_id))
}
