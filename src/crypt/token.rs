use super::{b64, decode_signature, error::Error, sign_content};
use crate::{
    config, keys,
    util::time::{now_utc, now_utc_plus_sec_str, parse_utc},
};
use rsa::{
    pkcs1v15::{SigningKey, VerifyingKey},
    sha2::Sha512,
    signature::Verifier,
};
use std::{fmt::Display, str::FromStr};
use uuid::Uuid;

#[derive(Debug)]
pub struct Token {
    pub identifier: String, // can be anything, an identifier number, an UUID, an unique username, etc
    pub expiration: String,
    pub signature: String, // base64_url_safe
}

impl FromStr for Token {
    type Err = Error;
    fn from_str(token_str: &str) -> Result<Self, Self::Err> {
        let splits: Vec<&str> = token_str.split('.').collect();
        if splits.len() != 3 {
            return Err(Error::TokenInvalidFormat);
        }

        let (identifier_b64u, expiration_b64u, signature_b64u) = (splits[0], splits[1], splits[2]);

        Ok(Self {
            identifier: b64::decode_to_string(identifier_b64u)?,
            expiration: b64::decode_to_string(expiration_b64u)?,
            signature: signature_b64u.to_string(),
        })
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}",
            b64::encode(&self.identifier),
            b64::encode(&self.expiration),
            &self.signature
        )
    }
}

impl Token {
    pub fn validate(&self, key: &VerifyingKey<Sha512>) -> Result<(), Error> {
        let content = format!(
            "{}.{}",
            b64::encode(&self.identifier),
            b64::encode(&self.expiration)
        );
        let signature = decode_signature(self.signature.as_str())?;

        // validates signature
        key.verify(content.as_bytes(), &signature)
            .map_err(|_| Error::InvalidTokenSignature)?;

        // checks expiration
        let expiration_time = parse_utc(self.expiration.as_str())?;
        let now = now_utc();

        if expiration_time < now {
            return Err(Error::ExpiredTokenError);
        }

        Ok(())
    }

    pub fn new_access_token(user: &str) -> Result<Self, Error> {
        let duration = &config().access_token_duration_secs;
        let key = &keys().signing_key;

        let token = Self::new(user, duration, key)?;

        Ok(token)
    }

    pub fn new_refresh_token() -> Result<Self, Error> {
        let duration = &config().refresh_token_duration_secs;
        let key = &keys().signing_key;

        let token = Self::new(&Uuid::new_v4().to_string(), duration, key)?;

        Ok(token)
    }

    fn new(identifier: &str, duration_secs: &u64, key: &SigningKey<Sha512>) -> Result<Self, Error> {
        let identifier = identifier.to_string();
        let expiration = now_utc_plus_sec_str(*duration_secs)?;

        let content = format!("{}.{}", b64::encode(&identifier), b64::encode(&expiration));

        let signature = sign_content(content, key);

        Ok(Self {
            identifier,
            expiration,
            signature,
        })
    }
}
