use super::error::Result;
use crate::{api::error::Error, config, AppState};
use axum::{
    body::Body,
    extract::{Path, State},
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use tower::util::ServiceExt;
use tower_http::services::ServeFile;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/stream/:id", get(stream_handler))
        .with_state(state)
}

// TODO: Use ctx here
async fn stream_handler(
    State(_state): State<AppState>,
    Path(song_id): Path<String>,
    request: Request<Body>,
) -> Result<Response> {
    tracing::debug!("GET STREAM HANDLER");

    let media_path = format!("./{}/{}.flac", &config().yt_dlp_output_path, song_id);

    let serve_file = ServeFile::new(media_path)
        .oneshot(request)
        .await
        .map_err(|_| Error::IOError)?;

    if serve_file.status() != StatusCode::NOT_FOUND {
        Ok(serve_file.into_response())
    } else {
        Err(Error::FileNotFound)
    }
}
