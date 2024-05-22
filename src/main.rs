use anyhow::Result;
use ripfy_server::{build_app, config, db, keys, AppState};
use std::net::SocketAddr;
use tokio::net::TcpListener;
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

    tracing::info!("Connecting to database...");
    let db = db::connect().await?;
    tracing::info!("Creating redis client...");
    let redis_client = redis::Client::open(config().redis_url.as_str())?;

    let state = AppState { db, redis_client };

    let app = build_app(state);

    let socket_address = SocketAddr::from(([0, 0, 0, 0], config().port));
    let listener = TcpListener::bind(&socket_address).await?;

    tracing::info!("Listening on {}", socket_address);

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
