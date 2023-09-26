pub mod conf;
pub mod context;
pub mod db;
pub mod middleware;
pub mod routes;

pub use conf::config;

use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}
