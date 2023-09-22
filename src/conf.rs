use figment::{
    providers::{Format, Serialized, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub db_location: String,
    pub port: u16,
}

// fallback default values for each config
impl Default for Config {
    fn default() -> Self {
        Config {
            db_location: "ripfy.sqlite".into(),
            port: 7717,
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Figment::from(Serialized::defaults(Config::default()))
            .merge(Toml::file("conf.toml"))
            .extract()
            .unwrap()
    }
}
