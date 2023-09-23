use anyhow::Result;
use ripfy_server::config;

#[tokio::test]
async fn client_mock() -> Result<()> {
    let client = httpc_test::new_client(format!("http://localhost:{}", config().port))?;

    client.do_get("/").await?.print().await?;

    Ok(())
}
