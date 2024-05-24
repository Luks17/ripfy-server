use anyhow::Result;
use axum::http::StatusCode;
use dev_utils::{spawn_test_app, util::get_port};
use ripfy_server::api::{ResponseModel, ResponseModelAuth};
use serde_json::json;

#[tokio::test]
async fn song_exclusivity_integration_test() -> Result<()> {
    let port = get_port();
    spawn_test_app(port, true).await?;

    let mut client_one = httpc_test::new_client(format!("http://localhost:{}", port))?;
    let mut client_two = httpc_test::new_client(format!("http://localhost:{}", port))?;

    client_one.add_auth_header(
        httpc_test::AuthHeaderType::Bearer,
        client_one
            .post::<ResponseModelAuth>(
                "/api/login",
                json!({
                "username": "demo1",
                "pwd": "demo1passwd"
                }),
            )
            .await?
            .data
            .unwrap()
            .access_token,
    )?;

    client_two.add_auth_header(
        httpc_test::AuthHeaderType::Bearer,
        client_two
            .post::<ResponseModelAuth>(
                "/api/login",
                json!({
                "username": "demo2",
                "pwd": "demo2passwd"
                }),
            )
            .await?
            .data
            .unwrap()
            .access_token,
    )?;

    client_one
        .do_post(
            "/api/songs",
            json!({
                "link": "https://www.youtube.com/watch?v=fJ9rUzIMcZQ"
                }
            ),
        )
        .await?;

    let get_song_status = client_one.do_get("/api/songs/fJ9rUzIMcZQ").await?.status();
    assert_eq!(get_song_status, StatusCode::OK.as_u16());

    let get_song_status = client_two.do_get("/api/songs/fJ9rUzIMcZQ").await?.status();
    assert_eq!(get_song_status, StatusCode::NOT_FOUND.as_u16());

    Ok(())
}

#[tokio::test]
async fn song_existance_integration_test() -> Result<()> {
    let port = get_port();
    spawn_test_app(port, true).await?;

    let mut client = httpc_test::new_client(format!("http://localhost:{}", port))?;

    let queen_classics_songs = ["fJ9rUzIMcZQ", "2ZBtPf7FOoM"];

    client.add_auth_header(
        httpc_test::AuthHeaderType::Bearer,
        client
            .post::<ResponseModelAuth>(
                "/api/login",
                json!({
                "username": "demo1",
                "pwd": "demo1passwd"
                }),
            )
            .await?
            .data
            .unwrap()
            .access_token,
    )?;

    let get_song_status = client
        .do_get(format!("/api/songs/{}", queen_classics_songs[0]).as_str())
        .await?
        .status();

    // asserts first queen song was not inserted yet
    assert_eq!(get_song_status, StatusCode::NOT_FOUND.as_u16());

    // adds queen songs
    for song in queen_classics_songs.iter() {
        client
            .do_post(
                "/api/songs",
                json!({
                    "link": format!("https://youtu.be/{}", song)
                    }
                ),
            )
            .await?;
    }

    let get_song_status = client
        .do_get(format!("/api/songs/{}", queen_classics_songs[0]).as_str())
        .await?
        .status();

    // asserts first queen song was added
    assert_eq!(get_song_status, StatusCode::OK.as_u16());

    let retrieved_songs: Vec<String> = client
        .get::<ResponseModel<Vec<entity::song::Model>>>("/api/songs")
        .await?
        .data
        .unwrap()
        .iter()
        .map(|song| song.id.clone())
        .collect();

    // asserts all queen songs were inserted
    for song in queen_classics_songs.iter() {
        assert!(retrieved_songs.contains(&song.to_string()));
    }

    // deletes first queen song
    client
        .do_delete(format!("/api/songs/{}", queen_classics_songs[0]).as_str())
        .await?;

    let get_song_status = client
        .do_get(format!("/api/songs/{}", queen_classics_songs[0]).as_str())
        .await?
        .status();

    // asserts first queen song was deleted
    assert_eq!(get_song_status, StatusCode::NOT_FOUND.as_u16());

    Ok(())
}
