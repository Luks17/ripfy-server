use super::junctions;
use crate::AppState;
use entity::{playlist_song, song, user_song};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseBackend, DbErr, EntityTrait, JoinType,
    QueryFilter, QuerySelect, QueryTrait, RelationTrait,
};

/// Finds a song entity that is related by user_song to an user entity and Returns it
///
/// Requires the AppState, SongId and the UserId of the User that made the request
///
/// Return sea_orm::DbErr if the SELECT operation fails
pub async fn first_by_id(
    state: &AppState,
    song_id: &str,
    user_id: &str,
) -> Result<Option<song::Model>, DbErr> {
    let db = &state.db;

    // Equivalent to:
    //
    // SELECT song.*
    // FROM song
    // JOIN user_song ON song.id = user_song.song_id
    // WHERE song.id = $input_song_id AND user_song.user_id = $input_user_id;
    let song = song::Entity::find_by_id(song_id)
        .join(JoinType::LeftJoin, song::Relation::UserSong.def())
        .filter(user_song::Column::UserId.eq(user_id))
        .one(db)
        .await?;

    Ok(song)
}

pub async fn all_from_playlist(
    state: &AppState,
    playlist_id: &str,
) -> Result<Vec<song::Model>, DbErr> {
    let db = &state.db;

    let songs = song::Entity::find()
        .join(JoinType::LeftJoin, song::Relation::PlaylistSong.def())
        .filter(playlist_song::Column::PlaylistId.eq(playlist_id))
        .all(db)
        .await?;

    Ok(songs)
}

pub async fn all_from_user(state: &AppState, user_id: &str) -> Result<Vec<song::Model>, DbErr> {
    let db = &state.db;

    let songs = song::Entity::find()
        .join(JoinType::LeftJoin, song::Relation::UserSong.def())
        .filter(user_song::Column::UserId.eq(user_id))
        .all(db)
        .await?;

    Ok(songs)
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
    };

    let new_song = new_song.insert(db).await?;

    junctions::user_song::create_new(state, user_id, link_id).await?;

    Ok(new_song)
}
