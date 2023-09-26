use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("An authentication token is required in the request header!")]
    AuthTokenRequired,
    #[error("Token provided in header is in the wrong format! Use 'user-[user-id].[expiration].[signature]'.")]
    AuthTokenWrongFormat,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (StatusCode::FORBIDDEN, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}
