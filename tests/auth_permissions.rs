use anyhow::Result;
use axum::http::StatusCode;
use dev_utils::build_test_app;
use ripfy_server::config;
use serde_json::json;

#[tokio::test]
async fn auth_permissions_integration_test() -> Result<()> {
    build_test_app(false).await?;

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
