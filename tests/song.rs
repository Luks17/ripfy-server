use anyhow::Result;
use axum::http::StatusCode;
use dev_utils::{spawn_test_app, util::get_port};
use serde_json::json;

#[tokio::test]
async fn song_exclusivity_integration_test() -> Result<()> {
    let port = get_port();
    spawn_test_app(port, true).await?;

    let client_one = httpc_test::new_client(format!("http://localhost:{}", port))?;
    let client_two = httpc_test::new_client(format!("http://localhost:{}", port))?;

    client_one
        .do_post(
            "/api/login",
            json!({
            "username": "demo1",
            "pwd": "demo1passwd"
            }),
        )
        .await?;

    client_two
        .do_post(
            "/api/login",
            json!({
            "username": "demo2",
            "pwd": "demo2passwd"
            }),
        )
        .await?;

    client_one
        .do_post(
            "/api/songs",
            json!({
                "link": "https://www.youtube.com/watch?v=fJ9rUzIMcZQ"
                }
            ),
        )
        .await?;

    let get_song_status = client_one.do_get("/api/song/fJ9rUzIMcZQ").await?.status();
    assert_eq!(get_song_status, StatusCode::OK);

    let get_song_status = client_two.do_get("/api/song/fJ9rUzIMcZQ").await?.status();
    assert_eq!(get_song_status, StatusCode::NOT_FOUND);

    Ok(())
}

#[tokio::test]
async fn song_existance_integration_test() -> Result<()> {
    let port = get_port();
    spawn_test_app(port, true).await?;

    let client = httpc_test::new_client(format!("http://localhost:{}", port))?;

    client
        .do_post(
            "/api/login",
            json!({
            "username": "demo1",
            "pwd": "demo1passwd"
            }),
        )
        .await?;

    let get_song_status = client.do_get("/api/song/fJ9rUzIMcZQ").await?.status();
    assert_eq!(get_song_status, StatusCode::NOT_FOUND);

    client
        .do_post(
            "/api/songs",
            json!({
                "link": "https://www.youtube.com/watch?v=fJ9rUzIMcZQ"
                }
            ),
        )
        .await?;

    let get_song_status = client.do_get("/api/song/fJ9rUzIMcZQ").await?.status();
    assert_eq!(get_song_status, StatusCode::OK);

    client.do_delete("/api/song/fJ9rUzIMcZQ").await?;

    let get_song_status = client.do_get("/api/song/fJ9rUzIMcZQ").await?.status();
    assert_eq!(get_song_status, StatusCode::NOT_FOUND);

    Ok(())
}
