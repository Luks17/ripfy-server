use anyhow::Result;
use axum::http::StatusCode;
use dev_utils::build_test_app;
use ripfy_server::config;
use serde_json::json;

#[tokio::test]
async fn song_existance_integration_test() -> Result<()> {
    build_test_app(true).await?;

    let client = httpc_test::new_client(format!("http://localhost:{}", config().port))?;

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
