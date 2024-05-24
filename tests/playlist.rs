use anyhow::{anyhow, Result};
use axum::http::StatusCode;
use dev_utils::{spawn_test_app, util::get_port};
use ripfy_server::api::{ResponseModel, ResponseModelAuth};
use serde_json::json;

#[tokio::test]
async fn playlist_songs_insertion_deletion_integration_test() -> Result<()> {
    let port = get_port();
    spawn_test_app(port, true).await?;

    let mut client = httpc_test::new_client(format!("http://localhost:{}", port))?;

    let queen_classics_songs = ["fJ9rUzIMcZQ", "2ZBtPf7FOoM"];
    let acdc_song = "Nnjh-zp6pP4";

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

    // song to test later
    client
        .do_post(
            "/api/songs",
            json!({
                "link": format!("https://youtu.be/{}", acdc_song)
                }
            ),
        )
        .await?;

    // creates playlist
    let playlist: ResponseModel<entity::playlist::Model> = client
        .do_post(
            "/api/playlists",
            json!({
            "title": "Queen Classics"
            }),
        )
        .await?
        .json_body_as()?;

    let playlist_data = playlist
        .data
        .ok_or_else(|| anyhow!("Could not unwrap playlist data"))?;

    // adds queen songs to playlist
    for song in queen_classics_songs.iter() {
        client
            .do_post(
                format!("/api/playlists/{}/songs", playlist_data.id).as_str(),
                json!({
                    "song_id": song
                    }
                ),
            )
            .await?;
    }

    let songs: Vec<String> = client
        .do_get(format!("/api/playlists/{}/songs", playlist_data.id).as_str())
        .await?
        .json_body_as::<ResponseModel<Vec<entity::song::Model>>>()?
        .data
        .ok_or_else(|| anyhow!("Could not unwrap playlist songs data"))?
        .iter()
        .map(|song| song.id.clone())
        .collect();

    // asserts all queen songs were inserted
    for song in queen_classics_songs.iter() {
        assert!(songs.contains(&song.to_string()));
    }

    // adds acdc song to playlist by 'mistake'
    client
        .do_post(
            format!("/api/playlists/{}/songs", playlist_data.id).as_str(),
            json!({
                "song_id": acdc_song
                }
            ),
        )
        .await?;

    // asserts it was actually added to the playlist
    assert!(client
        .do_get(format!("/api/playlists/{}/songs", playlist_data.id).as_str())
        .await?
        .json_body_as::<ResponseModel<Vec<entity::song::Model>>>()?
        .data
        .ok_or_else(|| anyhow!("Could not unwrap playlist songs data"))?
        .iter()
        .map(|song| song.id.as_str())
        .collect::<Vec<&str>>()
        .contains(&acdc_song));

    client
        .do_delete(format!("/api/playlists/{}/songs/{}", playlist_data.id, acdc_song).as_str())
        .await?;

    // asserts acdc song was deleted
    assert!(!client
        .do_get(format!("/api/playlists/{}/songs", playlist_data.id).as_str())
        .await?
        .json_body_as::<ResponseModel<Vec<entity::song::Model>>>()?
        .data
        .ok_or_else(|| anyhow!("Could not unwrap playlist songs data"))?
        .iter()
        .map(|song| song.id.as_str())
        .collect::<Vec<&str>>()
        .contains(&acdc_song));

    // deletes playlist
    client
        .do_delete(format!("/api/playlists/{}", playlist_data.id).as_str())
        .await?;

    // asserts it was deleted
    assert!(client
        .do_get("/api/playlists")
        .await?
        .json_body_as::<ResponseModel<Vec<entity::song::Model>>>()?
        .data
        .ok_or_else(|| anyhow!("Could not unwrap playlists data"))?
        .is_empty());

    // asserts that an error is returned when trying to get songs from it
    assert_eq!(
        client
            .do_get(format!("/api/playlists/{}/songs", playlist_data.id).as_str())
            .await?
            .status()
            .as_u16(),
        StatusCode::NOT_FOUND
    );

    Ok(())
}

#[tokio::test]
async fn playlist_exclusivity_integration_test() -> Result<()> {
    let port = get_port();
    spawn_test_app(port, true).await?;

    let mut client_one = httpc_test::new_client(format!("http://localhost:{}", port))?;
    let mut client_two = httpc_test::new_client(format!("http://localhost:{}", port))?;

    let queen_song = "fJ9rUzIMcZQ";

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
                "link": format!("https://youtu.be/{}", queen_song)
                }
            ),
        )
        .await?;

    // creates playlist
    let playlist: ResponseModel<entity::playlist::Model> = client_one
        .do_post(
            "/api/playlists",
            json!({
            "title": "Queen Classics"
            }),
        )
        .await?
        .json_body_as()?;

    let playlist_data = playlist
        .data
        .ok_or_else(|| anyhow!("Could not unwrap playlist data"))?;

    // adds queen song to playlist
    client_one
        .do_post(
            format!("/api/playlists/{}/songs", playlist_data.id).as_str(),
            json!({
                "song_id": queen_song
                }
            ),
        )
        .await?;

    // asserts it was actually added to the playlist by client one
    assert!(client_one
        .do_get(format!("/api/playlists/{}/songs", playlist_data.id).as_str())
        .await?
        .json_body_as::<ResponseModel<Vec<entity::song::Model>>>()?
        .data
        .ok_or_else(|| anyhow!("Could not unwrap playlists songs data"))?
        .iter()
        .map(|song| song.id.as_str())
        .collect::<Vec<&str>>()
        .contains(&queen_song));

    // asserts client two does not have the playlist (should return empty json)
    assert!(client_two
        .do_get("/api/playlists")
        .await?
        .json_body_as::<ResponseModel<Vec<entity::playlist::Model>>>()?
        .data
        .ok_or_else(|| anyhow!("Could not unwrap playlists data"))?
        .is_empty());

    // asserts client two cannot access the playlist songs (should return error)
    assert_eq!(
        client_two
            .do_get(format!("/api/playlists/{}/songs", playlist_data.id).as_str())
            .await?
            .status()
            .as_u16(),
        StatusCode::NOT_FOUND
    );

    // asserts client two cannot delete client one playlist
    assert_eq!(
        client_two
            .do_delete(format!("/api/playlists/{}", playlist_data.id).as_str())
            .await?
            .status()
            .as_u16(),
        StatusCode::NOT_FOUND
    );

    // asserts client two cannot delete client one playlist_song
    assert_eq!(
        client_two
            .do_delete(format!("/api/playlists/{}/songs/{}", playlist_data.id, queen_song).as_str())
            .await?
            .status()
            .as_u16(),
        StatusCode::NOT_FOUND
    );

    Ok(())
}

#[tokio::test]
async fn user_song_deletion_integration_test() -> Result<()> {
    let port = get_port();
    spawn_test_app(port, true).await?;

    let mut client = httpc_test::new_client(format!("http://localhost:{}", port))?;
    let acdc_song = "Nnjh-zp6pP4";

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

    // adds song for user
    client
        .do_post(
            "/api/songs",
            json!({
                "link": format!("https://youtu.be/{}", acdc_song)
                }
            ),
        )
        .await?;

    let playlist: ResponseModel<entity::playlist::Model> = client
        .do_post(
            "/api/playlists",
            json!({
            "title": "Queen Classics"
            }),
        )
        .await?
        .json_body_as()?;

    let playlist_data = playlist
        .data
        .ok_or_else(|| anyhow!("Could not unwrap playlist data"))?;

    // adds song to playlist
    client
        .do_post(
            format!("/api/playlists/{}/songs", playlist_data.id).as_str(),
            json!({
                "song_id": acdc_song
                }
            ),
        )
        .await?;

    // asserts it was actually added to the playlist
    assert!(client
        .do_get(format!("/api/playlists/{}/songs", playlist_data.id).as_str())
        .await?
        .json_body_as::<ResponseModel<Vec<entity::song::Model>>>()?
        .data
        .ok_or_else(|| anyhow!("Could not unwrap playlist songs data"))?
        .iter()
        .map(|song| song.id.as_str())
        .collect::<Vec<&str>>()
        .contains(&acdc_song));

    // removes song from user
    client
        .do_delete(format!("/api/songs/{}", acdc_song).as_str())
        .await?;

    // asserts it is no longer in the playlist
    assert!(!client
        .do_get(format!("/api/playlists/{}/songs", playlist_data.id).as_str())
        .await?
        .json_body_as::<ResponseModel<Vec<entity::song::Model>>>()?
        .data
        .ok_or_else(|| anyhow!("Could not unwrap playlist songs data"))?
        .iter()
        .map(|song| song.id.as_str())
        .collect::<Vec<&str>>()
        .contains(&acdc_song));

    Ok(())
}
