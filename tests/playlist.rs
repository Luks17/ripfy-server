use anyhow::Result;
use dev_utils::{spawn_test_app, util::get_port};
use ripfy_server::api::ModelResponse;
use serde_json::json;

#[tokio::test]
async fn playlist_songs_insertion_deletion_integration_test() -> Result<()> {
    let port = get_port();
    spawn_test_app(port, true).await?;

    let client = httpc_test::new_client(format!("http://localhost:{}", port))?;

    let queen_classics_songs = ["fJ9rUzIMcZQ", "2ZBtPf7FOoM"];
    let acdc_song = "Nnjh-zp6pP4";

    client
        .do_post(
            "/api/login",
            json!({
            "username": "demo1",
            "pwd": "demo1passwd"
            }),
        )
        .await?;

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
    let playlist: ModelResponse<entity::playlist::Model> = client
        .do_post(
            "/api/playlists",
            json!({
            "title": "Queen Classics"
            }),
        )
        .await?
        .json_body_as()?;

    // adds queen songs to playlist
    for song in queen_classics_songs.iter() {
        client
            .do_post(
                format!("/api/playlists/{}/songs", playlist.data.id).as_str(),
                json!({
                    "song_id": song
                    }
                ),
            )
            .await?;
    }

    let songs: Vec<String> = client
        .do_get(format!("/api/playlists/{}/songs", playlist.data.id).as_str())
        .await?
        .json_body_as::<ModelResponse<Vec<entity::song::Model>>>()?
        .data
        .iter()
        .map(|song| song.id.clone())
        .collect();

    // asserts all queen songs were inserted
    for song in queen_classics_songs.iter() {
        assert!(songs.contains(&song.to_string()))
    }

    // adds acdc song to playlist by 'mistake'
    client
        .do_post(
            format!("/api/playlists/{}/songs", playlist.data.id).as_str(),
            json!({
                "song_id": acdc_song
                }
            ),
        )
        .await?;

    // asserts it was actually added to the playlist
    assert!(client
        .do_get(format!("/api/playlists/{}/songs", playlist.data.id).as_str())
        .await?
        .json_body_as::<ModelResponse<Vec<entity::song::Model>>>()?
        .data
        .iter()
        .map(|song| song.id.as_str())
        .collect::<Vec<&str>>()
        .contains(&acdc_song));

    client
        .do_delete(format!("/api/playlists/{}/songs/{}", playlist.data.id, acdc_song).as_str())
        .await?;

    // asserts acdc song was deleted
    assert!(!client
        .do_get(format!("/api/playlists/{}/songs", playlist.data.id).as_str())
        .await?
        .json_body_as::<ModelResponse<Vec<entity::song::Model>>>()?
        .data
        .iter()
        .map(|song| song.id.as_str())
        .collect::<Vec<&str>>()
        .contains(&acdc_song));

    Ok(())
}

#[tokio::test]
async fn playlist_exclusivity() -> Result<()> {
    Ok(())
}
