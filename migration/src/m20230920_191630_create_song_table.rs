use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Song::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Song::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Song::Title).string().not_null())
                    .col(
                        ColumnDef::new(Song::Downloads)
                            .integer()
                            .not_null()
                            .default(1),
                    )
                    .col(ColumnDef::new(Song::ArtistId).integer().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Song::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Song {
    Table,
    Id,
    Title,
    Downloads,
    ArtistId,
}
