use super::{
    error::{Error, Result},
    ModelResponse,
};
use crate::{
    context::Ctx,
    db,
    util::{
        link::parse_yt_link,
        yt_dlp::{YtDlp, YtDlpResult},
    },
    AppState,
};
use axum::{
    extract::{Path, State},
    routing::{delete, get, post},
    Json, Router,
};
use entity::song::Model as Song;
use serde::Deserialize;
use serde_json::{json, Value};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/song/:id", get(get_song_handler))
        .route("/songs", post(add_song_handler))
        .route("/song/:id", delete(remove_song_handler))
        .with_state(state)
}

async fn get_song_handler(
    State(state): State<AppState>,
    _ctx: Ctx,
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
/// Also creates a junction table that links the song to the user that requested it
///
/// If the song already exists, no new song is created or actually downloaded, but instead the song
/// is just linked to the user by a junction table
async fn add_song_handler(
    State(state): State<AppState>,
    ctx: Ctx,
    Json(payload): Json<SongPayload>,
) -> Result<Json<Value>> {
    tracing::debug!("ADD SONG HANDLER");

    let SongPayload { link } = payload;
    let id = parse_yt_link(&link).map_err(|e| Error::InvalidPayload(e.to_string()))?;

    let song_option = db::song::first_by_id(&state, &id)
        .await
        .map_err(|_| Error::DbSelectFailed)?;

    // Exits early and creates user_song junction table for the song and user that requested it
    if let Some(song) = song_option {
        db::junctions::user_song::create_new(&state, &ctx.user_id(), &id)
            .await
            .map_err(|_| Error::DbInsertFailed)?;

        return Ok(Json(json!(ModelResponse {
            data: Song { ..song },
        })));
    }

    let process = YtDlp::default();
    let YtDlpResult { channel, fulltitle } = process
        .run_no_download(&id)
        .await
        .map_err(|e| Error::YtDlpError(e.to_string()))?;

    let new_song = db::song::create_new(&state, &id, &fulltitle, &channel, &ctx.user_id())
        .await
        .map_err(|_| Error::DbInsertFailed)?;

    Ok(Json(json!(ModelResponse {
        data: Song { ..new_song }
    })))
}

/// It's a soft delete, because it only removes user_song junction table, does not actually remove
/// song table or song file
async fn remove_song_handler(
    State(state): State<AppState>,
    ctx: Ctx,
    Json(payload): Json<SongPayload>,
) -> Result<Json<Value>> {
    tracing::debug!("REMOVE SONG HANDLER");

    let SongPayload { link: song_id } = payload;

    // removes junction table
    db::junctions::user_song::delete(&state, &ctx.user_id(), &song_id)
        .await
        .map_err(|_| Error::DbDeleteFailed)?;

    Ok(Json(json!(
        {
        "result": "success"
        }
    )))
}

#[derive(Debug, Deserialize)]
struct SongPayload {
    link: String,
}
