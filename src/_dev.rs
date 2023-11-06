use crate::{build_app, config, db, keys, AppState};
use anyhow::Result;
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use std::net::SocketAddr;

/// Used for integration tests
pub async fn build_test_app(use_demo_users: bool, addr: SocketAddr) -> Result<()> {
    tracing::info!("BUILDING TEST APP");

    let db = Database::connect("sqlite::memory:").await?;

    Migrator::up(&db, None).await?;

    let state = AppState { db };

    if use_demo_users {
        demo_users(&state).await?;
    }

    config();
    keys();

    tokio::spawn(async move {
        axum::Server::bind(&addr)
            .serve(build_app(state).into_make_service())
            .await?;

        Ok::<(), anyhow::Error>(())
    });

    tracing::info!("Listening on {}", addr);

    Ok(())
}

async fn demo_users(state: &AppState) -> Result<()> {
    db::user::create_new_user(state, "demo1", "demo1passwd").await?;
    db::user::create_new_user(state, "demo2", "demo2passwd").await?;
    db::user::create_new_user(state, "demo3", "demo3passwd").await?;

    Ok(())
}
