use anyhow::Result;
use axum::http::StatusCode;
use dev_utils::{spawn_test_app, util::get_port};
use ripfy_server::api::ResponseModelAuth;
use serde_json::json;

#[tokio::test]
async fn auth_permissions_integration_test() -> Result<()> {
    let port = get_port();
    spawn_test_app(port, false).await?;

    let mut client = httpc_test::new_client(format!("http://localhost:{}", port))?;

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

    client.add_auth_header(
        httpc_test::AuthHeaderType::Bearer,
        client
            .post::<ResponseModelAuth>(
                "/api/login",
                json!({
                "username": "user",
                "pwd": "passwd"
                }),
            )
            .await?
            .data
            .unwrap()
            .access_token,
    )?;

    let add_song = client.do_post(
        "/api/songs",
        json!({
        "link": "https://youtu.be/fJ9rUzIMcZQ?si=RfOiwzgyIWE6XQb9"
        }),
    );

    assert_eq!(add_song.await?.status(), StatusCode::OK.as_u16());

    Ok(())
}

#[tokio::test]
async fn auth_tokens_usage_integration_test() -> Result<()> {
    let port = get_port();
    spawn_test_app(port, true).await?;

    let mut client = httpc_test::new_client(format!("http://localhost:{}", port))?;

    let tokens = client
        .post::<ResponseModelAuth>(
            "/api/login",
            json!({
            "username": "demo1",
            "pwd": "demo1passwd"
            }),
        )
        .await?
        .data
        .unwrap();

    client.add_auth_header(httpc_test::AuthHeaderType::Bearer, tokens.access_token)?;

    assert_eq!(
        client
            .do_post(
                "/api/songs",
                json!({
                    "link": "https://www.youtube.com/watch?v=fJ9rUzIMcZQ"
                    }
                ),
            )
            .await?
            .status()
            .as_u16(),
        StatusCode::OK
    );

    client.add_auth_header(httpc_test::AuthHeaderType::Bearer, tokens.refresh_token)?;

    assert_eq!(
        client
            .do_post(
                "/api/songs",
                json!({
                    "link": "https://www.youtube.com/watch?v=fJ9rUzIMcZQ"
                    }
                ),
            )
            .await?
            .status()
            .as_u16(),
        StatusCode::UNAUTHORIZED
    );

    Ok(())
}

#[tokio::test]
async fn auth_tokens_refresh_integration_test() -> Result<()> {
    let port = get_port();
    spawn_test_app(port, true).await?;

    let client = httpc_test::new_client(format!("http://localhost:{}", port))?;

    let tokens = client
        .post::<ResponseModelAuth>(
            "/api/login",
            json!({
            "username": "demo1",
            "pwd": "demo1passwd"
            }),
        )
        .await?
        .data
        .unwrap();

    assert_eq!(
        client
            .do_post(
                "/api/refresh-token",
                json!({
                    "auth_token": tokens.access_token
                }),
            )
            .await?
            .status()
            .as_u16(),
        StatusCode::BAD_REQUEST
    );

    let new_refresh_token = client
        .post::<ResponseModelAuth>(
            "/api/refresh-token",
            json!({
                "auth_token": tokens.refresh_token
            }),
        )
        .await?
        .data
        .unwrap()
        .refresh_token;

    assert_eq!(
        client
            .do_post(
                "/api/refresh-token",
                json!({
                    "auth_token": tokens.refresh_token
                })
            )
            .await?
            .status()
            .as_u16(),
        StatusCode::BAD_REQUEST
    );

    assert_eq!(
        client
            .do_post(
                "/api/refresh-token",
                json!({
                    "auth_token": new_refresh_token
                })
            )
            .await?
            .status()
            .as_u16(),
        StatusCode::OK
    );

    Ok(())
}
