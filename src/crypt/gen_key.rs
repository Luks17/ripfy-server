use super::error::Error;
use rsa::{
    pkcs1::{EncodeRsaPrivateKey, LineEnding},
    RsaPrivateKey,
};
use std::fs;

const KEY_PATH: &str = "key.pem";

pub fn gen_key_if_not_exists() -> Result<(), Error> {
    if fs::metadata(KEY_PATH).is_err() {
        let mut rng = rand::thread_rng();

        let bits = 1024;
        let private_key = RsaPrivateKey::new(&mut rng, bits).map_err(|_| Error::KeyGenFailed)?;

        private_key
            .write_pkcs1_pem_file(KEY_PATH, LineEnding::LF)
            .map_err(|_| Error::KeyGenFailed)?
    }

    Ok(())
}
