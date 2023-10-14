use super::error::Error;
use crate::config;
use rsa::{
    pkcs1::{EncodeRsaPrivateKey, LineEnding},
    RsaPrivateKey,
};
use std::fs;

/// Generates a RSA Private Key if it does not already exists at private_key_path
pub fn gen_private_key_if_not_exists() -> Result<(), Error> {
    if fs::metadata(&config().private_key_path).is_err() {
        let mut rng = rand::thread_rng();

        let bits = 1024;
        let private_key = RsaPrivateKey::new(&mut rng, bits).map_err(|_| Error::KeyGenFailed)?;

        private_key
            .write_pkcs1_pem_file(&config().private_key_path, LineEnding::LF)
            .map_err(|_| Error::KeyGenFailed)?
    }

    Ok(())
}
