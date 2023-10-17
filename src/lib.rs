pub mod api;
pub mod conf;
pub mod context;
pub mod crypt;
pub mod db;
pub mod util;

pub use conf::config;
pub use conf::keys;

use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}
