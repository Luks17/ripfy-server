use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, FromQueryResult, Serialize, Deserialize, ToSchema)]
pub struct PlaylistModel {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub songs_number: i64,
}
