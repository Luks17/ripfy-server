pub mod conf;
pub mod db;

use conf::Config;
use lazy_static::lazy_static;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

lazy_static! {
    pub static ref CONF: Config = Config::new();
}
