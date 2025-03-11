use super::migration_trait::Migrationable;
use crate::helper::sql_connection_helper::create_connection;
use crate::model::database_metadata::{self, DatabaseMetadata};
use crate::model::{product::Product, user::User};
use diesel::RunQueryDsl;
use sea_query::SqliteQueryBuilder;

pub fn migrate_up() {
    // TODO: only do migrate when current version is more than db version
    let conn = &mut create_connection();

    let up_migrations = [
        Product::get_up_migration(SqliteQueryBuilder),
        User::get_up_migration(SqliteQueryBuilder),
    ];

    up_migrations.iter().for_each(|item| {
        let result = diesel::sql_query(item).execute(conn);
        result.expect("error when migrate up!");
    });
}

pub fn migration_down() {
    // TODO: only do migrate when current version is more than db version
    let conn = &mut create_connection();

    let up_migrations = [
        Product::get_down_migration(SqliteQueryBuilder),
        User::get_up_migration(SqliteQueryBuilder),
    ];

    up_migrations.iter().for_each(|item| {
        let result = diesel::sql_query(item).execute(conn);
        result.expect("error when migrate down!");
    });
}

pub fn migrate_database_metadata() {
    let conn = &mut create_connection();
    let create_db_metadata = DatabaseMetadata::get_up_migration(SqliteQueryBuilder);
    let result = diesel::sql_query(create_db_metadata).execute(conn);
    result.expect("Error When trying to migrate database metadata!!!");
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_up_migration() {
        migrate_up();
    }

    #[test]
    fn test_down_migration() {
        migration_down();
    }
}
