use anyhow::Result;
use axum::{routing, Router, Server};
use ripfy_server::{db, AppState, CONF};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)?;

    let db = db::connect().await?;

    let state = AppState { db };

    let app = Router::new()
        .route("/", routing::get(|| async { "Hello, World!" }))
        .with_state(state);

    let socket_address = SocketAddr::from(([0, 0, 0, 0], CONF.port));
    tracing::info!("Listening on {}", socket_address);

    Server::bind(&socket_address)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
