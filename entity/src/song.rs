//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "song")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub title: String,
    pub channel: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user_song::Entity")]
    UserSong,
}

impl Related<super::user_song::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserSong.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        super::user_song::Relation::User.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::user_song::Relation::Song.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
