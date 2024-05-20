use super::error::{Error, Result};
use crate::{context::Ctx, crypt::token::Token, keys};
use async_trait::async_trait;
use axum::{
    body::Body,
    extract::FromRequestParts,
    http::{request::Parts, Request},
    middleware::Next,
    response::Response,
    RequestExt,
};
use axum_auth::AuthBearer;

// This middleware is useful to restrict access to routes only to authenticated users
// When an extractor is wrapped in Result, axum will not immediately reject the request if it does
// not match
pub async fn ctx_require_auth(
    ctx: Result<Ctx>,
    request: Request<Body>,
    next: Next,
) -> Result<Response> {
    tracing::debug!("MIDDLEWARE - REQUIRE_AUTHENTICATION_CTX");

    ctx?;

    Ok(next.run(request).await)
}

/// Middleware for extracting bearer token from authorization request header and returning a context
/// Also refreshes token if valid or removes it if invalid
pub async fn ctx_resolver(mut req: Request<Body>, next: Next) -> Result<Response> {
    tracing::debug!("MIDDLEWARE - CTX_RESOLVER");

    let ctx = extract_and_parse_token(&mut req).await;

    // Store the ctx_result in the request extension.
    req.extensions_mut().insert(ctx);

    Ok(next.run(req).await)
}

async fn extract_and_parse_token(req: &mut Request<Body>) -> Result<Ctx> {
    let token: String = req
        .extract_parts::<AuthBearer>()
        .await
        .map(|AuthBearer(bearer)| bearer)
        .map_err(|_| Error::NoAuthToken)?;

    // if the token exists and the parse is successful, the token is then validated
    let token: Token = token.parse()?;
    token.validate(&keys().verifying_key)?;

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
