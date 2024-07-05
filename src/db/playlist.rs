use entity::{playlist, playlist_song};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DbErr, EntityTrait, JoinType,
    QueryFilter, QuerySelect, RelationTrait,
};

use crate::{
    api::queries::playlist::PlaylistQuery, db::results::playlist::PlaylistModel, AppState,
};

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
    query_params: PlaylistQuery,
) -> Result<Vec<PlaylistModel>, DbErr> {
    let db = &state.db;

    let mut query = playlist::Entity::find()
        .join(JoinType::LeftJoin, playlist::Relation::PlaylistSong.def())
        .column_as(playlist_song::Column::PlaylistId.count(), "songs_number")
        .filter(playlist::Column::UserId.eq(user_id));

    if let Some(search_str) = query_params.search {
        query = query.filter(
            Condition::any().add(playlist::Column::Title.like(format!("%{}%", &search_str))),
        );
    }

    query.into_model::<PlaylistModel>().all(db).await
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
