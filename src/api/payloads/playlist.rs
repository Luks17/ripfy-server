use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct PlaylistPayload {
    pub title: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct PlaylistSongPayload {
    pub song_id: String,
}
