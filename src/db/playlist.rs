use entity::playlist;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, DbErr, EntityTrait, QueryFilter};

use crate::AppState;

pub async fn first_by_id(
    state: &AppState,
    playlist_id: &str,
    user_id: &str,
) -> Result<Option<playlist::Model>, DbErr> {
    let db = &state.db;

    let playlist = playlist::Entity::find_by_id(playlist_id)
        .filter(playlist::Column::UserId.eq(user_id))
        .one(db)
        .await?;

    Ok(playlist)
}

pub async fn all_by_user_id(
    state: &AppState,
    user_id: &str,
) -> Result<Vec<playlist::Model>, DbErr> {
    let db = &state.db;

    let playlists = playlist::Entity::find()
        .filter(playlist::Column::UserId.eq(user_id))
        .all(db)
        .await?;

    Ok(playlists)
}

pub async fn create_new(
    state: &AppState,
    user_id: &str,
    title: &str,
) -> Result<playlist::Model, DbErr> {
    let db = &state.db;

    let new_playlist = playlist::ActiveModel {
        id: ActiveValue::Set(uuid::Uuid::new_v4().to_string()),
        user_id: ActiveValue::Set(user_id.into()),
        title: ActiveValue::Set(title.into()),
    };

    let new_playlist = new_playlist.insert(db).await?;

    Ok(new_playlist)
}

pub async fn delete(state: &AppState, playlist_id: &str) -> Result<(), DbErr> {
    let db = &state.db;

    playlist::Entity::delete_by_id(playlist_id).exec(db).await?;

    Ok(())
}
