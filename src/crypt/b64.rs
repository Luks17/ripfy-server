use super::error::Error;
use base64::{engine::general_purpose, DecodeError, Engine};

pub fn encode(data: impl AsRef<[u8]>) -> String {
    general_purpose::URL_SAFE_NO_PAD.encode(data)
}

pub fn decode(data: &str) -> Result<Vec<u8>, Error> {
    Ok(general_purpose::URL_SAFE_NO_PAD.decode(data)?)
}

pub fn decode_to_string(data: &str) -> Result<String, Error> {
    let vector = decode(data)?;
    let string = String::from_utf8(vector).map_err(|_| DecodeError::InvalidPadding)?;

    Ok(string)
}
