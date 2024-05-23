use entity::user;
use sea_orm::{ActiveValue, ColumnTrait, DbErr, EntityTrait, QueryFilter};

use crate::AppState;

pub async fn first_by_username(
    state: &AppState,
    username: &str,
) -> Result<Option<user::Model>, DbErr> {
    let db = &state.db;

    let user = user::Entity::find()
        .filter(user::Column::Username.eq(username))
        .one(db)
        .await?;

    Ok(user)
}

pub async fn create_new_user(state: &AppState, username: &str, passwd: &str) -> Result<(), DbErr> {
    let db = &state.db;

    let new_user = user::ActiveModel {
        id: ActiveValue::Set(uuid::Uuid::new_v4().to_string()),
        username: ActiveValue::Set(username.to_string()),
        passwd: ActiveValue::Set(passwd.to_string()),
    };

    user::Entity::insert(new_user).exec(db).await?;

    Ok(())
}
