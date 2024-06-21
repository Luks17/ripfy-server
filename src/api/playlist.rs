use super::error::{Error, Result};
use crate::{
    api::{
        payloads::playlist::{PlaylistPayload, PlaylistSongPayload},
        ResponseModel, ResponseModelPlaylist,
    },
    context::Ctx,
    db, AppState,
};
use axum::{
    extract::{Path, State},
    routing::{delete, get, post},
    Json, Router,
};
use entity::{playlist::Model as Playlist, song::Model as Song};
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
#[utoipa::path(
    get,
    path = "/api/playlists",
    responses(
        (status = 200, description = "Success loading all playlists", body = ResponseModel,
            example = json!(ResponseModel::<Vec<Playlist>> {
                success: true,
                data: Some(vec![
                    Playlist {id: "rf2z5v".into(), user_id: "lka934".into(), title: "Queen classics".into()},
                    Playlist {id: "lma3zt".into(), user_id: "lka934".into(), title: "Best of the 80s".into()}
                ]),
                error: None
            }))
    )
)]
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

#[utoipa::path(
    get,
    path = "/api/playlists/{id}/songs",
    params(("id" = String, Path, description = "Playlist id")),
    responses(
        (status = 200, description = "Success loading playlist", body = ResponseModel,
            example = json!(ResponseModel::<Vec<Song>> {
                success: true,
                data: Some(vec![
                    Song {id: "13rca0z".into(), title: "High Voltage".into(), channel: "AC/DC".into()},
                    Song {id: "dkefj2c".into(), title: "Highway to Hell".into(), channel: "AC/DC".into()},
                    Song {id: "mefiae5".into(), title: "Thunderstruck".into(), channel: "AC/DC".into()},
                ]),
                error: None
            }))
    )
)]
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

    Ok(Json(json!(ResponseModel {
        success: true,
        data: Some(songs),
        error: None
    })))
}

#[utoipa::path(
    post,
    path = "/api/playlists",
    request_body = PlaylistPayload,
    responses(
        (status = 200, description = "Success create playlist", body = ResponseModelPlaylist,
            example = json!(ResponseModelPlaylist {
                success: true,
                data: Some(Playlist { id: "aek143z".into(), title: "Bestof the 80s".into(), user_id: "la2sa9x".into() }),
                error: None
            }))
    )
)]
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

    Ok(Json(json!(ResponseModelPlaylist {
        success: true,
        data: Some(Playlist { ..new_playlist }),
        error: None
    })))
}

#[utoipa::path(
    post,
    path = "/api/playlists/{id}/songs",
    params(("id" = String, Path, description = "Playlist id")),
    request_body = PlaylistSongPayload,
    responses(
        (status = 200, description = "Success add song in playlist", body = ResponseModel,
            example = json!(ResponseModel::<()> { success: true, data: None, error: None }))
    )
)]
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

#[utoipa::path(
    delete,
    path = "/api/playlists/{id}",
    params(("id" = String, Path, description = "Playlist id")),
    request_body = PlaylistSongPayload,
    responses(
        (status = 200, description = "Success deleting playlist", body = ResponseModel,
            example = json!(ResponseModel::<()> { success: true, data: None, error: None }))
    )
)]
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

#[utoipa::path(
    delete,
    path = "/api/playlists/{playlist_id}/songs/{song_id}",
    params(("playlist_id" = String, Path, description = "Playlist id"), ("song_id" = String, Path, description = "Playlist song id")),
    responses(
        (status = 200, description = "Success deleting song in playlist", body = ResponseModel,
            example = json!(ResponseModel::<()> { success: true, data: None, error: None }))
    )
)]
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
