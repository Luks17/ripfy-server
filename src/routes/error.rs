use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Error, Clone, Debug)]
pub enum Error {
    #[error("Could not login user!")]
    LoginFailed,
    #[error("No authentication token was provided in the request header!")]
    NoAuthToken,
    #[error("Token provided in header is in the wrong format!\nExpected the following format: 'user-[user-id].[expiration].[signature]'.")]
    AuthTokenWrongFormat,
    #[error("The context is missing from the request extension! Something may have gone wrong on the token validation.")]
    CtxNotInRequestExtensions,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        tracing::debug!("ERROR INTO_RESPONSE - {self:?}");

        let (status_code, client_error) = self.parse_server_error_to_client();

        let client_error_body = json!({
            "error": {
                "type": client_error.as_ref(),
            }
        });

        tracing::debug!("SENT CLIENT ERROR: {client_error_body}");

        (status_code, Json(client_error_body)).into_response()
    }
}

impl Error {
    /// Converts server error to client error and status code
    /// This method main purpose is to not send sensitive information to the client
    pub fn parse_server_error_to_client(&self) -> (StatusCode, ClientError) {
        #[allow(unreachable_patterns)]
        match self {
            Self::LoginFailed => (StatusCode::UNAUTHORIZED, ClientError::LOGIN_FAIL),
            Self::NoAuthToken | Self::AuthTokenWrongFormat | Self::CtxNotInRequestExtensions => {
                (StatusCode::UNAUTHORIZED, ClientError::NO_AUTH)
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}
