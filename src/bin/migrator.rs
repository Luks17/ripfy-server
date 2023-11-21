use anyhow::Result;
use migration::{Migrator, MigratorTrait};
use ripfy_server::{config, db};

#[tokio::main]
async fn main() -> Result<()> {
    config();

    let db = db::connect().await?;

    Migrator::up(&db, None).await?;

    Ok(())
}
