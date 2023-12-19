use crate::AppState;
use entity::user_song;
use sea_orm::{ActiveModelTrait, ActiveValue, DbErr, EntityTrait};

/// Creates a new UserSong junction table that associates an user with a song
/// Returns Ok(()) when successful and sea_orm::DbErr when INSERT fails
pub async fn create_new(state: &AppState, user_id: &str, song_id: &str) -> Result<(), DbErr> {
    let db = &state.db;

    let new_user_song = user_song::ActiveModel {
        user_id: ActiveValue::Set(user_id.to_string()),
        song_id: ActiveValue::Set(song_id.to_string()),
        ..Default::default()
    };

    new_user_song.insert(db).await?;

    Ok(())
}

pub async fn delete(state: &AppState, user_id: &str, song_id: &str) -> Result<(), DbErr> {
    let db = &state.db;

    let pk = (user_id.to_string(), song_id.to_string());

    user_song::Entity::delete_by_id(pk).exec(db).await?;

    Ok(())
}
