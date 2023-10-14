use super::{
    super::{gen_and_set_token_cookie, remove_token_cookie},
    error::{Error, Result},
    AUTH_TOKEN,
};
use crate::{context::Ctx, crypt::token::Token, keys};
use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, Request},
    middleware::Next,
    response::Response,
};
use tower_cookies::Cookies;

/// Middleware for extracting token cookie from request header and returning a context
/// Also refreshes token if valid or removes it if invalid
pub async fn ctx_resolver<B>(
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    tracing::debug!("MIDDLEWARE - CTX_RESOLVER");

    let ctx = verify_and_refresh_token(&cookies).await;

    // If the client sends an invalid cookie, we want to remove it
    // these if statements take care of it
    if let Err(ref e) = ctx {
        if !matches!(*e, Error::NoAuthToken) {
            tracing::debug!("MIDDLEWARE - CTX_RESOLVER - REMOVING INVALID COOKIE FROM HEADER");
            remove_token_cookie(&cookies).await;
        }
    }

    // Store the ctx_result in the request extension.
    req.extensions_mut().insert(ctx);

    Ok(next.run(req).await)
}

async fn verify_and_refresh_token(cookies: &Cookies) -> Result<Ctx> {
    // extracts auth token from cookies as a string
    let token_str = cookies
        .get(AUTH_TOKEN)
        .map(|c| c.value().to_string())
        .ok_or(Error::NoAuthToken)?;

    // if the token exists and the parse is successful, the token is then validated
    let token: Token = token_str.parse()?;
    token.validate(&keys().verifying_key)?;

    // refreshes access token
    gen_and_set_token_cookie(cookies, &token.identifier).await?;

    Ok(Ctx::new(&token.identifier))
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
