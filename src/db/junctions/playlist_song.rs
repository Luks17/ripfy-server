use entity::playlist_song;
use sea_orm::{ActiveModelTrait, ActiveValue, DbErr, EntityTrait};

use crate::AppState;

pub async fn create_new(state: &AppState, playlist_id: &str, song_id: &str) -> Result<(), DbErr> {
    let db = &state.db;

    let new_playlist_song = playlist_song::ActiveModel {
        playlist_id: ActiveValue::Set(playlist_id.into()),
        song_id: ActiveValue::Set(song_id.into()),
        ..Default::default()
    };

    new_playlist_song.insert(db).await?;

    Ok(())
}

pub async fn delete(state: &AppState, playlist_id: &str, song_id: &str) -> Result<(), DbErr> {
    let db = &state.db;

    let pk = (playlist_id.to_string(), song_id.to_string());

    playlist_song::Entity::delete_by_id(pk).exec(db).await?;

    Ok(())
}
