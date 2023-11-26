use anyhow::Result;
use axum::http::StatusCode;
use dev_utils::spawn_test_app;
use ripfy_server::config;
use serde_json::json;

#[tokio::test]
async fn song_exclusivity_integration_test() -> Result<()> {
    spawn_test_app(true).await?;

    let client_one = httpc_test::new_client(format!("http://localhost:{}", config().port))?;
    let client_two = httpc_test::new_client(format!("http://localhost:{}", config().port))?;

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
