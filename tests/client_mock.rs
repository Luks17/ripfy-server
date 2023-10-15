use anyhow::Result;
use ripfy_server::config;
use serde_json::json;

#[tokio::test]
#[ignore]
async fn client_mock() -> Result<()> {
    let client = httpc_test::new_client(format!("http://localhost:{}", config().port))?;

    client.do_get("/").await?.print().await?;

    let signup = client.do_post(
        "/api/signup",
        json!({
        "username": "user",
        "pwd": "passwd",
        }),
    );

    signup.await?.print().await?;

    let login = client.do_post(
        "/api/login",
        json!({
        "username": "user",
        "pwd": "passwd"
        }),
    );

    login.await?.print().await?;

    client.do_get("/").await?.print().await?;

    let logout = client.do_post(
        "/api/logout",
        json!({
        "logoff": true
        }),
    );

    logout.await?.print().await?;

    client.do_get("/").await?.print().await?;

    Ok(())
}
