use crate::util;
use base64::DecodeError;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Something went wrong while decoding b64 to string!")]
    DecodingError(#[from] DecodeError),
    #[error("Token provided in header is in the wrong format!\nExpected the following format: 'user-[user-id].[expiration].[signature]'.")]
    TokenInvalidFormat,
    #[error("Could not generate private key!")]
    KeyGenFailed,
    #[error("An error ocurred while trying to parse bytes of signature to Signature!")]
    SignParsingFailed,
    #[error(transparent)]
    FailedToGetTime(#[from] util::error::Error),
    #[error("The provided token identifier is not a valid uuid")]
    InvalidTokenIdentifier,
    #[error("The provided token content does not match with the signature!")]
    InvalidTokenSignature,
    #[error("The provided token is expired!")]
    ExpiredTokenError,
    #[error("The provided token is not an access_token!")]
    NotAnAccessTokenError,
}
