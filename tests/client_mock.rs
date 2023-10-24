use anyhow::Result;
use ripfy_server::config;
use serde_json::json;

#[tokio::test]
#[ignore]
async fn client_mock() -> Result<()> {
    let client = httpc_test::new_client(format!("http://localhost:{}", config().port))?;

    let add_song = client.do_post(
        "/api/songs",
        json!({
        "link": "https://youtu.be/fJ9rUzIMcZQ?si=RfOiwzgyIWE6XQb9"
        }),
    );

    add_song.await?.print().await?;

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

    let add_song = client.do_post(
        "/api/songs",
        json!({
        "link": "https://youtu.be/fJ9rUzIMcZQ?si=RfOiwzgyIWE6XQb9"
        }),
    );

    add_song.await?.print().await?;

    let logout = client.do_post(
        "/api/logout",
        json!({
        "logoff": true
        }),
    );

    logout.await?.print().await?;

    Ok(())
}
