use super::error::{Error, Result};
use crate::{api::ResponseModel, context::Ctx, db, AppState};
use axum::{
    extract::{Path, State},
    routing::{delete, get, post},
    Json, Router,
};
use entity::playlist::Model as Playlist;
use serde::Deserialize;
use serde_json::{json, Value};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/playlists", get(get_playlists_handler))
        .route("/playlists/:id/songs", get(get_playlist_songs_handler))
        .route("/playlists", post(create_playlist_handler))
        .route("/playlists/:id/songs", post(add_playlist_song_handler))
        .route("/playlists/:id", delete(delete_playlist_handler))
        .route(
            "/playlists/:playlist_id/songs/:song_id",
            delete(delete_playlist_song_handler),
        )
        .with_state(state)
}

/// Returns playlists created by user if any exists
///
/// WILL NOT return a playlist owned by another user
async fn get_playlists_handler(State(state): State<AppState>, ctx: Ctx) -> Result<Json<Value>> {
    tracing::debug!("GET PLAYLISTS HANDLER");

    let playlists = db::playlist::all_by_user_id(&state, &ctx.user_id())
        .await
        .map_err(|_| Error::DbSelectFailed)?;

    Ok(Json(json!(ResponseModel {
        success: true,
        data: Some(playlists),
        error: None
    })))
}

async fn get_playlist_songs_handler(
    State(state): State<AppState>,
    ctx: Ctx,
    Path(id): Path<String>,
) -> Result<Json<Value>> {
    tracing::debug!("GET PLAYLIST SONGS HANDLER");

    // makes sure the playlist exists and is owned by user
    db::playlist::first_by_id(&state, &id, &ctx.user_id())
        .await
        .map_err(|_| Error::DbSelectFailed)?
        .ok_or(Error::PlaylistNotFound)?;

    let songs = db::song::all_from_playlist(&state, &id)
        .await
        .map_err(|_| Error::DbSelectFailed)?;

    // TODO: does not have type assertion, implement later
    Ok(Json(json!(ResponseModel {
        success: true,
        data: Some(songs),
        error: None
    })))
}

async fn create_playlist_handler(
    State(state): State<AppState>,
    ctx: Ctx,
    Json(payload): Json<PlaylistPayload>,
) -> Result<Json<Value>> {
    tracing::debug!("CREATE PLAYLIST HANDLER");

    let PlaylistPayload { title } = payload;

    let new_playlist = db::playlist::create_new(&state, &ctx.user_id(), &title)
        .await
        .map_err(|_| Error::DbInsertFailed)?;

    Ok(Json(json!(ResponseModel {
        success: true,
        data: Some(Playlist { ..new_playlist }),
        error: None
    })))
}

async fn add_playlist_song_handler(
    State(state): State<AppState>,
    ctx: Ctx,
    Path(playlist_id): Path<String>,
    Json(payload): Json<PlaylistSongPayload>,
) -> Result<Json<Value>> {
    tracing::debug!("ADD PLAYLIST_SONG HANDLER");

    let PlaylistSongPayload { song_id } = payload;

    // makes sure the playlist exists and is owned by user
    db::playlist::first_by_id(&state, &playlist_id, &ctx.user_id())
        .await
        .map_err(|_| Error::DbSelectFailed)?
        .ok_or(Error::PlaylistNotFound)?;

    db::junctions::playlist_song::create_new(&state, &playlist_id, &song_id)
        .await
        .map_err(|_| Error::DbInsertFailed)?;

    Ok(Json(json!(ResponseModel::<()> {
        success: true,
        data: None,
        error: None
    })))
}

async fn delete_playlist_handler(
    State(state): State<AppState>,
    ctx: Ctx,
    Path(id): Path<String>,
) -> Result<Json<Value>> {
    tracing::debug!("DELETE PLAYLIST HANDLER");

    // makes sure the playlist exists and is owned by user
    db::playlist::first_by_id(&state, &id, &ctx.user_id())
        .await
        .map_err(|_| Error::DbSelectFailed)?
        .ok_or(Error::PlaylistNotFound)?;

    db::playlist::delete(&state, &id)
        .await
        .map_err(|_| Error::DbDeleteFailed)?;

    Ok(Json(json!(ResponseModel::<()> {
        success: true,
        data: None,
        error: None
    })))
}

async fn delete_playlist_song_handler(
    State(state): State<AppState>,
    ctx: Ctx,
    Path((playlist_id, song_id)): Path<(String, String)>,
) -> Result<Json<Value>> {
    tracing::debug!("DELETE PLAYLIST SONG HANDLER");

    // makes sure the playlist exists and is owned by user
    db::playlist::first_by_id(&state, &playlist_id, &ctx.user_id())
        .await
        .map_err(|_| Error::DbSelectFailed)?
        .ok_or(Error::PlaylistNotFound)?;

    db::junctions::playlist_song::delete(&state, &playlist_id, &song_id)
        .await
        .map_err(|_| Error::DbDeleteFailed)?;

    Ok(Json(json!(ResponseModel::<()> {
        success: true,
        data: None,
        error: None
    })))
}

#[derive(Debug, Deserialize)]
struct PlaylistPayload {
    title: String,
}

#[derive(Debug, Deserialize)]
struct PlaylistSongPayload {
    song_id: String,
}
