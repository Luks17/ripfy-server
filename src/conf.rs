use figment::{
    providers::{Format, Serialized, Toml},
    Figment,
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
    GLOBAL_CONF.get_or_init(|| Config::new())
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub db_location: String,
    pub access_token_duration_secs: u32,
    pub refresh_token_duration_secs: u32,
    pub port: u16,
}

// fallback default values for each config
impl Default for Config {
    fn default() -> Self {
        Config {
            db_location: "ripfy.sqlite".into(),
            access_token_duration_secs: 1800,    // 30 minutes
            refresh_token_duration_secs: 604800, // 1 week
            port: 7717,
        }
    }
}

impl Config {
    fn new() -> Self {
        Figment::from(Serialized::defaults(Config::default()))
            .merge(Toml::file("conf.toml"))
            .extract()
            .unwrap()
    }
}
