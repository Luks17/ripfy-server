use anyhow::Result;
use ripfy_server::config;
use serde_json::json;

#[tokio::test]
async fn client_mock() -> Result<()> {
    let client = httpc_test::new_client(format!("http://localhost:{}", config().port))?;

    client.do_get("/").await?.print().await?;

    let login = client.do_post(
        "/api/login",
        json!({
        "username": "user",
        "pwd": "passwd",
        }),
    );

    login.await?.print().await?;

    Ok(())
}
