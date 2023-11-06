use anyhow::Result;
use axum::http::StatusCode;
use ripfy_server::{_dev::build_test_app, config};
use serde_json::json;
use std::net::SocketAddr;
use tracing_subscriber::EnvFilter;

#[tokio::test]
async fn auth_permissions_integration_test() -> Result<()> {
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let socket_address = SocketAddr::from(([0, 0, 0, 0], config().port));
    build_test_app(false, socket_address).await?;

    let client = httpc_test::new_client(format!("http://localhost:{}", config().port))?;

    let add_song = client.do_post(
        "/api/songs",
        json!({
        "link": "https://youtu.be/fJ9rUzIMcZQ?si=RfOiwzgyIWE6XQb9"
        }),
    );

    assert_eq!(add_song.await?.status(), StatusCode::UNAUTHORIZED);

    let signup = client.do_post(
        "/api/signup",
        json!({
        "username": "user",
        "pwd": "passwd",
        }),
    );

    signup.await?;

    let login = client.do_post(
        "/api/login",
        json!({
        "username": "user",
        "pwd": "passwd"
        }),
    );

    assert_eq!(login.await?.status(), StatusCode::OK);

    let add_song = client.do_post(
        "/api/songs",
        json!({
        "link": "https://youtu.be/fJ9rUzIMcZQ?si=RfOiwzgyIWE6XQb9"
        }),
    );

    assert_eq!(add_song.await?.status(), StatusCode::OK);

    let logout = client.do_post(
        "/api/logout",
        json!({
        "logoff": true
        }),
    );

    logout.await?;

    let add_song = client.do_post(
        "/api/songs",
        json!({
        "link": "https://youtu.be/fJ9rUzIMcZQ?si=RfOiwzgyIWE6XQb9"
        }),
    );

    assert_eq!(add_song.await?.status(), StatusCode::UNAUTHORIZED);

    Ok(())
}
