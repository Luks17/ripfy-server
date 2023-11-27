pub mod util;

use crate::util::start_global_subscriber;
use anyhow::Result;
use migration::{Migrator, MigratorTrait};
use ripfy_server::{
    build_app, config,
    crypt::passwd::{gen_salt, passwd_encrypt},
    db, keys, AppState,
};
use sea_orm::Database;
use std::net::SocketAddr;

/// Used for integration tests
pub async fn spawn_test_app(port: u16, use_demo_users: bool) -> Result<()> {
    start_global_subscriber();

    tracing::info!("BUILDING TEST APP");

    let db = Database::connect("sqlite::memory:").await?;

    Migrator::up(&db, None).await?;

    let state = AppState { db };

    if use_demo_users {
        demo_users(&state).await?;
    }

    config();
    keys();

    let socket_addr = SocketAddr::from(([0, 0, 0, 0], port));

    tokio::spawn(async move {
        axum::Server::bind(&socket_addr)
            .serve(build_app(state).into_make_service())
            .await?;

        Ok::<(), anyhow::Error>(())
    });

    tracing::info!("Listening on {}", socket_addr);

    Ok(())
}

async fn demo_users(state: &AppState) -> Result<()> {
    db::user::create_new_user(
        state,
        "demo1",
        passwd_encrypt("demo1passwd", gen_salt())?.as_str(),
    )
    .await?;

    db::user::create_new_user(
        state,
        "demo2",
        passwd_encrypt("demo2passwd", gen_salt())?.as_str(),
    )
    .await?;

    db::user::create_new_user(
        state,
        "demo3",
        passwd_encrypt("demo3passwd", gen_salt())?.as_str(),
    )
    .await?;

    Ok(())
}
