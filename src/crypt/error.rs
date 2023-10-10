use base64::DecodeError;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Something went wrong while decoding b64 to string!")]
    DecodingError(#[from] DecodeError),
    #[error("Token provided in header is in the wrong format!\nExpected the following format: 'user-[user-id].[expiration].[signature]'.")]
    TokenInvalidFormat,
}
