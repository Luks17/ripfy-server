use super::{
    error::{Error, Result},
    ModelResponse,
};
use crate::{
    db,
    util::{
        link::parse_yt_link,
        yt_dlp::{YtDlp, YtDlpResult},
    },
    AppState,
};
use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use entity::song::Model as Song;
use serde::Deserialize;
use serde_json::{json, Value};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/song/:id", get(get_song_handler))
        .route("/songs", post(add_song_handler))
        .with_state(state)
}

async fn get_song_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Value>> {
    tracing::debug!("GET SONG HANDLER");

    let song = match db::song::first_by_id(&state, &id).await {
        Ok(song) => song.ok_or(Error::SongNotFound)?,
        Err(_) => return Err(Error::DbSelectFailed),
    };

    Ok(Json(json!(ModelResponse {
        data: Song { ..song },
    })))
}

/// Tries to add a new song to the database and download it.
/// If the song already exists, the song download count increases by 1 and no new song is created
/// or actually downloaded
async fn add_song_handler(
    State(state): State<AppState>,
    Json(payload): Json<NewSongPayload>,
) -> Result<Json<Value>> {
    tracing::debug!("ADD SONG HANDLER");

    let NewSongPayload { link } = payload;
    let id = parse_yt_link(&link).map_err(|e| Error::InvalidPayload(e.to_string()))?;

    let song_option = db::song::first_by_id(&state, &id)
        .await
        .map_err(|_| Error::DbSelectFailed)?;

    // Download count for existing sound gets increased by 1 and exites handler early
    if let Some(song) = song_option {
        let song = db::song::add_or_subtract_downloads(&state, song, true)
            .await
            .map_err(|_| Error::DbUpdateFailded)?;

        return Ok(Json(json!(ModelResponse {
            data: Song { ..song },
        })));
    }

    let process = YtDlp::default();
    let YtDlpResult { channel, fulltitle } = process
        .run_no_download(&id)
        .await
        .map_err(|e| Error::YtDlpError(e.to_string()))?;

    let new_song = db::song::create_new_song(&state, &id, &fulltitle, &channel)
        .await
        .map_err(|_| Error::DbInsertFailed)?;

    Ok(Json(json!(ModelResponse {
        data: Song { ..new_song }
    })))
}

#[derive(Debug, Deserialize)]
struct NewSongPayload {
    link: String,
}
