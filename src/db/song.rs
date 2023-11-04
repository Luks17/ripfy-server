use super::junctions;
use crate::AppState;
use entity::song;
use sea_orm::{ActiveModelTrait, ActiveValue, DbErr, EntityTrait};

pub async fn first_by_id(state: &AppState, id: &str) -> Result<Option<song::Model>, DbErr> {
    let db = &state.db;

    let song = song::Entity::find_by_id(id).one(db).await?;

    Ok(song)
}

/// Creates a new song entity on the database and Returns it
/// Also creates a user_song junction entity
///
/// Requires the AppState, SongLinkId, SongTitle, SongChannel and the UserId of the User that made
/// the request
///
/// Returns sea_orm::DbErr if any INSERT operation fails
pub async fn create_new(
    state: &AppState,
    link_id: &str,
    title: &str,
    channel: &str,
    user_id: &str,
) -> Result<song::Model, DbErr> {
    let db = &state.db;

    let new_song = song::ActiveModel {
        id: ActiveValue::Set(link_id.to_string()),
        title: ActiveValue::Set(title.to_string()),
        channel: ActiveValue::Set(channel.to_string()),
        ..Default::default()
    };

    let new_song = new_song.insert(db).await?;

    junctions::user_song::create_new(state, user_id, link_id).await?;

    Ok(new_song)
}
