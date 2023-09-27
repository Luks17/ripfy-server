use super::error::Result;
use crate::context::Ctx;
pub use crate::middleware::AUTH_TOKEN;
use axum::{http::Request, middleware::Next, response::Response};

// This middleware is useful to restrict access to routes only to authenticated users
// When an extractor is wrapped in Result, axum will not immediately reject the request if it does
// not match
pub async fn authenticate<B>(
    ctx: Result<Ctx>,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    tracing::debug!("MIDDLEWARE - AUTHENTICATION");

    ctx?;

    Ok(next.run(request).await)
}