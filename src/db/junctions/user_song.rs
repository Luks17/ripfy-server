use crate::AppState;
use entity::{playlist, playlist_song, user_song};
use sea_orm::{
    sea_query::Query, ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DbErr, EntityTrait,
    QueryFilter,
};

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

    // deletes user song from all of his playlists
    playlist_song::Entity::delete_many()
        .filter(
            Condition::all()
                .add(
                    playlist_song::Column::PlaylistId.in_subquery(
                        Query::select()
                            .column(playlist::Column::Id)
                            .from(playlist::Entity)
                            .and_where(playlist::Column::UserId.eq(user_id))
                            .to_owned(),
                    ),
                )
                .add(playlist_song::Column::SongId.eq(song_id)),
        )
        .exec(db)
        .await?;

    Ok(())
}
