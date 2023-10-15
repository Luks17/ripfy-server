use entity::user;
use sea_orm::{ActiveValue, ColumnTrait, DbErr, EntityTrait, QueryFilter};

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

pub async fn create_new_user(
    state: &AppState,
    username: String,
    passwd: String,
) -> Result<(), DbErr> {
    let db = &state.db;

    let new_user = user::ActiveModel {
        id: ActiveValue::Set(uuid::Uuid::new_v4().to_string()),
        username: ActiveValue::Set(username),
        passwd: ActiveValue::Set(passwd),
    };

    user::Entity::insert(new_user).exec(db).await?;

    Ok(())
}
