use super::error::Error;
use base64::{engine::general_purpose, DecodeError, Engine};

pub fn encode(data: &str) -> String {
    general_purpose::URL_SAFE_NO_PAD.encode(data)
}

pub fn decode(data: &str) -> Result<String, Error> {
    let vector = general_purpose::URL_SAFE_NO_PAD.decode(data)?;
    let string = String::from_utf8(vector).map_err(|_| DecodeError::InvalidByte(0, 0))?;

    Ok(string)
}
