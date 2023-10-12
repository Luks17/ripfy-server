use super::{b64, error::Error};
use crate::{config, keys, util::time::now_utc_plus_sec_str};
use rsa::{pkcs1v15::SigningKey, sha2::Sha512};
use std::{fmt::Display, str::FromStr};

#[derive(Debug)]
pub struct Token {
    pub identifier: String,
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
            identifier: b64::decode(identifier_b64u)?,
            expiration: b64::decode(expiration_b64u)?,
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

pub fn generate_access_token(user: &str, salt: &str) {
    let duration = config().access_token_duration_secs;
    let key = &keys().signing_key;

    create_token(user, duration, salt, key)
}

fn create_token(identifier: &str, duration_secs: u64, _salt: &str, _key: &SigningKey<Sha512>) {
    let _identifier = identifier.to_string();
    let _exp = now_utc_plus_sec_str(duration_secs);
}
