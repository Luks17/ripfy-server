use crate::crypt::gen_key::gen_private_key_if_not_exists;
use anyhow::Result;
use figment::{
    providers::{Format, Serialized, Toml},
    Figment,
};
use rsa::{
    pkcs1::DecodeRsaPrivateKey,
    pkcs1v15::{SigningKey, VerifyingKey},
    sha2::Sha512,
    signature::Keypair,
    RsaPrivateKey,
};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

/// Creates or uses existing instance of config
/// Uses OnceLock to:
/// - Have one immutable and irreplacable referece of the instance of Config;
/// - Use an instance that will persist the entire code. Unlike OnceCell, OnceLock is thread-safe, so static references can be used.
pub fn config() -> &'static Config {
    static GLOBAL_CONF: OnceLock<Config> = OnceLock::new();

    // gets instace if exists, creates and gets it if not
    GLOBAL_CONF.get_or_init(|| {
        Config::new().unwrap_or_else(|err| panic!("FATAL - COULD NOT LOAD CONFIG - CAUSE: {err:?}"))
    })
}

pub fn keys() -> &'static Keys {
    static GLOBAL_KEYS: OnceLock<Keys> = OnceLock::new();

    GLOBAL_KEYS.get_or_init(|| {
        Keys::new().unwrap_or_else(|err| panic!("FATAL - COULD NOT LOAD KEYS - CAUSE: {err:?}"))
    })
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub db_location: String,
    pub private_key_path: String,
    pub access_token_duration_secs: u64,
    pub refresh_token_duration_secs: u64,
    pub yt_dlp_binary_path: String,
    pub yt_dlp_output_path: String,
    pub yt_dlp_timeout_milisecs: u64,
    pub port: u16,
}

// fallback default values for each config
impl Default for Config {
    fn default() -> Self {
        Config {
            db_location: "ripfy.sqlite".into(),
            private_key_path: "key.pem".into(),
            access_token_duration_secs: 1800,    // 30 minutes
            refresh_token_duration_secs: 604800, // 1 week
            yt_dlp_binary_path: "yt-dlp".into(), // default value assumes binary is on PATH
            yt_dlp_output_path: "media".into(),  // directory where media will be outputed
            yt_dlp_timeout_milisecs: 30000,      // 30 seconds
            port: 7717,
        }
    }
}

impl Config {
    fn new() -> Result<Self> {
        let c = Figment::from(Serialized::defaults(Config::default()))
            .merge(Toml::file("conf.toml"))
            .extract()?;

        Ok(c)
    }
}

pub struct Keys {
    pub signing_key: SigningKey<Sha512>,
    pub verifying_key: VerifyingKey<Sha512>,
}

impl Keys {
    fn new() -> Result<Self> {
        gen_private_key_if_not_exists()?;

        let private_key: RsaPrivateKey =
            DecodeRsaPrivateKey::read_pkcs1_pem_file(&config().private_key_path)?;
        let signing_key = SigningKey::<Sha512>::new(private_key);
        let verifying_key = signing_key.verifying_key();

        Ok(Keys {
            signing_key,
            verifying_key,
        })
    }
}
