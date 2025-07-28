use sea_orm_migration::{DbErr, SchemaManager};

pub trait Migrationable {
    fn get_up_migration(manager: &SchemaManager) -> Result<(), DbErr>;
    fn get_down_migration(manager: &SchemaManager) -> Result<(), DbErr>;
}
