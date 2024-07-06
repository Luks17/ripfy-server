pub mod auth;
mod error;
pub mod mw;
pub mod payloads;
pub mod playlist;
pub mod queries;
pub mod song;
pub mod stream;

use crate::db::results::playlist::PlaylistModel;
use entity::song::Model as Song;
use entity::user::Model as User;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[aliases(
    ResponseModelUser = ResponseModel<User>,
    ResponseModelSong = ResponseModel<Song>,
    ResponseModelPlaylist = ResponseModel<PlaylistModel>,
    ResponseModelAuth = ResponseModel<AuthModel>
)]
pub struct ResponseModel<T> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthModel {
    pub access_token: String,
    pub refresh_token: String,
}
