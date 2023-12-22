use anyhow::Result;
use ripfy_server::api::ModelResponse;
use serde_json::json;

#[tokio::test]
#[ignore]
async fn client_mock() -> Result<()> {
    let client = httpc_test::new_client(format!("http://localhost:{}", 7717))?;

    client
        .do_post(
            "/api/signup",
            json!({
            "username": "user",
            "pwd": "passwd",
            }),
        )
        .await?;

    client
        .do_post(
            "/api/login",
            json!({
            "username": "user",
            "pwd": "passwd"
            }),
        )
        .await?;

    client
        .do_post(
            "/api/songs",
            json!({
                "link": "https://www.youtube.com/watch?v=fJ9rUzIMcZQ"
                }
            ),
        )
        .await?;

    let playlist: ModelResponse<entity::playlist::Model> = client
        .do_post(
            "/api/playlists",
            json!({
            "title": "test"
            }),
        )
        .await?
        .json_body_as()?;

    Ok(())
}
