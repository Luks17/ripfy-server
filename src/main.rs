use anyhow::Result;
use axum::{middleware, Router, Server};
use ripfy_server::{api, config, db, keys, AppState};
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Since rust loads stuff lazily, we need to make sure the config and keys are available as early as possible,
    // because they can panic if not loaded correctly
    config();
    keys();

    let db = db::connect().await?;

    let state = AppState { db };

    let routes_rest = Router::new()
        .merge(api::song::router(state.clone()))
        .route_layer(middleware::from_fn(api::mw::ctx::ctx_require_auth));

    let app = Router::new()
        .nest("/api", api::auth::router(state.clone()))
        .nest("/api", routes_rest)
        .layer(middleware::from_fn(api::mw::ctx::ctx_resolver))
        .layer(CookieManagerLayer::new());

    let socket_address = SocketAddr::from(([0, 0, 0, 0], config().port));
    tracing::info!("Listening on {}", socket_address);

    Server::bind(&socket_address)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
