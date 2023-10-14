pub mod b64;
pub mod error;
pub mod gen_key;
pub mod passwd;
pub mod token;

use self::error::Error;
use rsa::{
    pkcs1v15::{Signature, SigningKey},
    sha2::Sha512,
    signature::{RandomizedSigner, SignatureEncoding},
};

/// Receives a base64url encoded String slice and tries to decode and extract a signature of it if exists
pub fn decode_signature(encoded_signature: &str) -> Result<Signature, Error> {
    let signature: &[u8] = &b64::decode(encoded_signature)?;
    let result = Signature::try_from(signature).map_err(|_| Error::SignParsingFailed)?;

    Ok(result)
}

/// Signs the content and return a base64url encoded String of the signature
pub fn sign_content(content: String, key: &SigningKey<Sha512>) -> String {
    let mut rng = rand::thread_rng();
    let raw_signature = key.sign_with_rng(&mut rng, content.as_bytes());

    b64::encode(raw_signature.to_bytes())
}
