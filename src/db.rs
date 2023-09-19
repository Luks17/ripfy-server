use std::{fs, time::Duration};

use anyhow::{Context, Result};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::CONF;

/// Tries to connect to existing database
pub async fn connect() -> Result<DatabaseConnection> {
    let db_location = get_db_file()?;

    let mut opt = ConnectOptions::new(format!("sqlite://{db_location}?mode=rwc"));
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(false);

    let db = Database::connect(opt)
        .await
        .with_context(|| "Could not connect to database!");

    db
}

/// Creates database file if does not exist
/// Returns a Result with the string of the location or an error if it was not possible to create a file
fn get_db_file() -> Result<String> {
    let db_location = CONF.db_location.as_str();

    if fs::metadata(db_location).is_err() {
        fs::File::create(db_location)
            .with_context(|| format!("Failed to create file: {}", db_location))?;
    }

    Ok(db_location.to_string())
}
