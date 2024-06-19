use super::{
    error::{Error, Result},
    ResponseModel,
};
use crate::{
    api::payloads::song::SongPayload,
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
use serde_json::{json, Value};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/songs", get(get_all_songs_handler))
        .route("/songs/:id", get(get_song_handler))
        .route("/songs", post(add_song_handler))
        .route("/songs/:id", delete(remove_song_handler))
        .with_state(state)
}

/// Returns all songs previously requested by user
///
/// WILL NOT return a song owned by another user
#[utoipa::path(
    get,
    path = "/api/songs",
    request_body = SongPayload,
    responses(
        (status = 200, description = "Success loading all music", body = ResponseModel,
            example = json!(ResponseModel { success: true, data: Some, error: None }))
    )
)]
async fn get_all_songs_handler(State(state): State<AppState>, ctx: Ctx) -> Result<Json<Value>> {
    tracing::debug!("GET ALL SONGS HANDLER");

    let songs = db::song::all_from_user(&state, &ctx.user_id())
        .await
        .map_err(|_| Error::DbSelectFailed)?;

    Ok(Json(json!(ResponseModel {
        success: true,
        data: Some(songs),
        error: None
    })))
}

/// Returns a song if the user that made the request previously requested it
///
/// WILL NOT return a song owned by another user
#[utoipa::path(
    get,
    path = "/api/songs/:id",
    request_body = SongPayload,
    responses(
        (status = 200, description = "Success loading music", body = ResponseModel,
            example = json!(ResponseModel ::<()> { success: true, data: Some, error: None })),
        (status = 500, description = "Failed to load music", body = ResponseModel,
            example = json!(ResponseModel ::<()> {success: false, data: None, error: Some(ClientError::SERVICE_ERROR.as_ref().to_string())}))
    )
)]
async fn get_song_handler(
    State(state): State<AppState>,
    ctx: Ctx,
    Path(id): Path<String>,
) -> Result<Json<Value>> {
    tracing::debug!("GET SONG HANDLER");

    let song = match db::song::first_by_id(&state, &id, &ctx.user_id()).await {
        Ok(song) => song.ok_or(Error::SongNotFound)?,
        Err(_) => return Err(Error::DbSelectFailed),
    };

    Ok(Json(json!(ResponseModel {
        success: true,
        data: Some(Song { ..song }),
        error: None
    })))
}

/// Tries to add a new song to the database and download it.
/// Also creates a junction table that links the song to the user that requested it, so the user
/// can "own" the song
///
/// If the song already exists, no new song is created or actually downloaded, but instead the song
/// is just linked to the user by a junction table
#[utoipa::path(
    post,
    path = "/api/songs",
    request_body = SongPayload,
    responses(
        (status = 200, description = "Success adding songs", body = ResponseModel,
            example = json!(ResponseModel ::<()> { success: true, data: Some, error: None })),
        (status = 200, description = "Success adding songs", body = ResponseModel,
            example = json!(ResponseModel ::<()> { success: true, data: Some, error: None }))
    )
)]
async fn add_song_handler(
    State(state): State<AppState>,
    ctx: Ctx,
    Json(payload): Json<SongPayload>,
) -> Result<Json<Value>> {
    tracing::debug!("ADD SONG HANDLER");

    let SongPayload { link } = payload;
    let song_id = parse_yt_link(&link).map_err(|e| Error::InvalidPayload(e.to_string()))?;

    let song_option = db::song::first_by_id(&state, &song_id, &ctx.user_id())
        .await
        .map_err(|_| Error::DbSelectFailed)?;

    // Exits early and creates user_song junction table for the song and user that requested it
    if let Some(song) = song_option {
        db::junctions::user_song::create_new(&state, &ctx.user_id(), &song_id)
            .await
            .map_err(|_| Error::DbInsertFailed)?;

        return Ok(Json(json!(ResponseModel {
            success: true,
            data: Some(Song { ..song }),
            error: None
        })));
    }

    let process = YtDlp::default();
    let YtDlpResult { channel, fulltitle } = process
        .run(&song_id)
        .await
        .map_err(|e| Error::YtDlpError(e.to_string()))?;

    let new_song = db::song::create_new(&state, &song_id, &fulltitle, &channel, &ctx.user_id())
        .await
        .map_err(|_| Error::DbInsertFailed)?;

    Ok(Json(json!(ResponseModel {
        success: true,
        data: Some(Song { ..new_song }),
        error: None
    })))
}

/// It's a soft delete, because it only removes user_song junction table, does not actually remove
/// song table or song file
#[utoipa::path(
    delete,
    path = "/api/songs/:id",
    request_body = SongPayload,
    responses(
        (status = 200, description = "Success adding songs", body = ResponseModel,
            example = json!(ResponseModel ::<()> { success: true, data: None, error: None }))
    )
)]
async fn remove_song_handler(
    State(state): State<AppState>,
    ctx: Ctx,
    Path(song_id): Path<String>,
) -> Result<Json<Value>> {
    tracing::debug!("REMOVE SONG HANDLER");

    // removes junction table
    db::junctions::user_song::delete(&state, &ctx.user_id(), &song_id)
        .await
        .map_err(|_| Error::DbDeleteFailed)?;

    Ok(Json(json!(ResponseModel::<()> {
        success: true,
        data: None,
        error: None
    })))
}
