use sea_orm_migration::prelude::*;

use crate::{m20230920_191630_create_song_table::Song, m20231008_182809_create_user::User};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserSong::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(UserSong::UserId).uuid().not_null())
                    .col(ColumnDef::new(UserSong::SongId).string().not_null())
                    .primary_key(Index::create().col(UserSong::UserId).col(UserSong::SongId))
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl(UserSong::Table)
                            .from_col(UserSong::UserId)
                            .to_tbl(User::Table)
                            .to_col(User::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl(UserSong::Table)
                            .from_col(UserSong::SongId)
                            .to_tbl(Song::Table)
                            .to_col(Song::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserSong::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum UserSong {
    Table,
    UserId,
    SongId,
}
