pub use sea_orm_migration::prelude::*;

mod m20230920_191630_create_song_table;
mod m20231008_182809_create_user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230920_191630_create_song_table::Migration),
            Box::new(m20231008_182809_create_user::Migration),
        ]
    }
}
