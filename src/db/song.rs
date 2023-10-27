use crate::AppState;
use entity::song;
use sea_orm::{ActiveModelTrait, ActiveValue, DbErr, EntityTrait};

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
