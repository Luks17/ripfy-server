use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Error, Clone, Debug)]
pub enum Error {
    #[error("Could not login user!")]
    LoginFailed,
    #[error("No authentication token was provided in the request header!")]
    NoAuthToken,
    #[error("Token provided in header is in the wrong format! Use 'user-[user-id].[expiration].[signature]'.")]
    AuthTokenWrongFormat,
    #[error("The context is missing from the request extension! Something may have gone wrong on the token validation.")]
    CtxNotInRequestExtensions,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        tracing::debug!("INTO_RESPONSE - {self:?}");
        (StatusCode::FORBIDDEN, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}
