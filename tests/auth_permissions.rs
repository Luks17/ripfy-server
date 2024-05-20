use anyhow::Result;
use axum::http::StatusCode;
use dev_utils::{spawn_test_app, util::get_port};
use serde_json::json;

#[tokio::test]
async fn auth_permissions_integration_test() -> Result<()> {
    let port = get_port();
    spawn_test_app(port, false).await?;

    let client = httpc_test::new_client(format!("http://localhost:{}", port))?;

    let add_song = client.do_post(
        "/api/songs",
        json!({
        "link": "https://youtu.be/fJ9rUzIMcZQ?si=RfOiwzgyIWE6XQb9"
        }),
    );

    assert_eq!(add_song.await?.status(), StatusCode::UNAUTHORIZED.as_u16());

    client
        .do_post(
            "/api/signup",
            json!({
            "username": "user",
            "pwd": "passwd",
            }),
        )
        .await?;

    let login = client.do_post(
        "/api/login",
        json!({
        "username": "user",
        "pwd": "passwd"
        }),
    );

    assert_eq!(login.await?.status(), StatusCode::OK.as_u16());

    let add_song = client.do_post(
        "/api/songs",
        json!({
        "link": "https://youtu.be/fJ9rUzIMcZQ?si=RfOiwzgyIWE6XQb9"
        }),
    );

    assert_eq!(add_song.await?.status(), StatusCode::OK.as_u16());

    Ok(())
}
