use anyhow::Result;
use migration::{Migrator, MigratorTrait};
use ripfy_server::{
    build_app, config,
    crypt::passwd::{gen_salt, passwd_encrypt},
    db, keys, AppState,
};
use sea_orm::Database;
use std::net::SocketAddr;
use tracing_subscriber::EnvFilter;

/// Used for integration tests
pub async fn spawn_test_app(use_demo_users: bool) -> Result<()> {
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    tracing::info!("BUILDING TEST APP");

    let db = Database::connect("sqlite::memory:").await?;

    Migrator::up(&db, None).await?;

    let state = AppState { db };

    if use_demo_users {
        demo_users(&state).await?;
    }

    config();
    keys();

    let socket_addr = SocketAddr::from(([0, 0, 0, 0], config().port));

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
