pub mod conf;
pub mod context;
pub mod db;
pub mod helpers;
pub mod routes;
pub mod util;

pub use conf::config;

use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}
