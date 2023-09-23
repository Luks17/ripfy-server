use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Could not login user!")]
    LoginFailed,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (StatusCode::FORBIDDEN, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}
