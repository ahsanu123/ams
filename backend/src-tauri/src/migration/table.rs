use diesel::{migration::Migration, RunQueryDsl};
use diesel_migrations::EmbeddedMigrations;
use sqlx::SqliteConnection;

pub const DB_VERSION: i32 = 1;

pub mod product_table;
pub mod table_trait;

pub fn migrate_database(conn: &mut SqliteConnection) {}

pub fn get_db_version() -> String {
    todo!()
}
