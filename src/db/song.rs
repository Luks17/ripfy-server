use crate::AppState;
use entity::song;
use sea_orm::{ActiveModelTrait, ActiveValue, DbErr, EntityTrait, Set};

pub async fn first_by_id(state: &AppState, id: &str) -> Result<Option<song::Model>, DbErr> {
    let db = &state.db;

    let song = song::Entity::find_by_id(id).one(db).await?;

    Ok(song)
}

pub async fn create_new_song(
    state: &AppState,
    link_id: &str,
    title: &str,
    channel: &str,
) -> Result<song::Model, DbErr> {
    let db = &state.db;

    let new_song = song::ActiveModel {
        id: ActiveValue::Set(link_id.to_string()),
        title: ActiveValue::Set(title.to_string()),
        channel: ActiveValue::Set(channel.to_string()),
        ..Default::default()
    };

    let new_song = new_song.insert(db).await?;

    Ok(new_song)
}

/// Receives a state, a song model and a boolean that will decide if the downloads count from the
/// model should be added to or subtracted from.
///
/// If add is true: downloads += 1; else: downloads -= 1.
///
/// Returns a DBErr when update fails;
/// If successful, returns updated song model.
pub async fn add_or_subtract_downloads(
    state: &AppState,
    track: song::Model,
    add: bool,
) -> Result<song::Model, DbErr> {
    let db = &state.db;
    let downloads = track.downloads.clone();
    let add = if add { 1 } else { -1 };

    let mut updated_track: song::ActiveModel = track.into();

    updated_track.downloads = Set(downloads + add);
    let track = updated_track.update(db).await?;

    Ok(track)
}
