use diesel::prelude::*;
use diesel::{migration::Migration as DieselMigration, RunQueryDsl};
use diesel_migrations::EmbeddedMigrations;
use sea_orm::{prelude::*, DeriveMigrationName};
use sea_orm_migration::{async_trait::async_trait, MigrationTrait, SchemaManager};
use sea_query::SqliteQueryBuilder;
use sqlx::{Pool, Sqlite, SqliteConnection};

use crate::helper::sql_connection_helper::create_connection;
use crate::model::product::Product;

use super::migration_trait::MigrationAble;

pub const DB_VERSION: i32 = 1;

pub mod product_table;
pub mod table_trait;

#[derive(DeriveMigrationName)]
pub struct MainMigrator;

#[async_trait]
impl MigrationTrait for MainMigrator {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let product_migration = Product::get_up_migration(SqliteQueryBuilder);

        let mut conn = create_connection();
        diesel::sql_query(&product_migration).execute(&mut conn);

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();
    }
}

// pub async fn migrate_database(conn: &mut SqliteConnection, manager: &SchemaManager) {}

pub fn get_db_version() -> String {
    todo!()
}

#[cfg(test)]
mod test {
    use sea_orm_migration::migrator;

    use super::*;

    #[test]
    fn check_table_migration() {
        let migrator = MainMigrator;
    }
}
