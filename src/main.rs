use anyhow::Result;
use axum::{middleware, routing, Router, Server};
use ripfy_server::{config, db, mw, routes, AppState};
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)?;

    let db = db::connect().await?;

    let state = AppState { db };

    let app = Router::new()
        .route("/", routing::get(|| async { "Hello, World!" }))
        .merge(routes::login::router(state.clone()))
        .layer(middleware::from_fn(mw::cookies::ctx_resolver))
        .layer(CookieManagerLayer::new());

    let socket_address = SocketAddr::from(([0, 0, 0, 0], config().port));
    tracing::info!("Listening on {}", socket_address);

    Server::bind(&socket_address)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
