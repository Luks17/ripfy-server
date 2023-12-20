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
                    .table(Playlist::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Playlist::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Playlist::UserId).uuid().not_null())
                    .col(ColumnDef::new(Playlist::Title).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl(Playlist::Table)
                            .from_col(Playlist::UserId)
                            .to_tbl(User::Table)
                            .to_col(User::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(PlaylistSong::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(PlaylistSong::PlaylistId).uuid().not_null())
                    .col(ColumnDef::new(PlaylistSong::SongId).string().not_null())
                    .col(
                        ColumnDef::new(PlaylistSong::AddedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .primary_key(
                        Index::create()
                            .col(PlaylistSong::PlaylistId)
                            .col(PlaylistSong::SongId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl(PlaylistSong::Table)
                            .from_col(PlaylistSong::PlaylistId)
                            .to_tbl(Playlist::Table)
                            .to_col(Playlist::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from_tbl(PlaylistSong::Table)
                            .from_col(PlaylistSong::SongId)
                            .to_tbl(Song::Table)
                            .to_col(Song::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Playlist::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Playlist {
    Table,
    Id,
    UserId,
    Title,
}

#[derive(DeriveIden)]
enum PlaylistSong {
    Table,
    PlaylistId,
    SongId,
    AddedAt,
}
