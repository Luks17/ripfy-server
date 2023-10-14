use entity::user;
use sea_orm::{ColumnTrait, DbErr, EntityTrait, QueryFilter};

use crate::AppState;

pub async fn first_by_username(
    state: &AppState,
    username: &str,
) -> Result<Option<user::Model>, DbErr> {
    let db = &state.db;

    let user = user::Entity::find()
        .filter(user::Column::Username.contains(username))
        .one(db)
        .await?;

    Ok(user)
}
