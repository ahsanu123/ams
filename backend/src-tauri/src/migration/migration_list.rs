use super::migration_trait::Migrationable;
use crate::{helper::sql_connection_helper::create_connection, model::product::Product};
use diesel::RunQueryDsl;
use sea_query::SqliteQueryBuilder;

pub fn migrate_up() {
    let conn = &mut create_connection();
    let up_migrations = vec![Product::get_up_migration(SqliteQueryBuilder)];

    up_migrations.iter().for_each(|item| {
        let result = diesel::sql_query(item).execute(conn);
        result.expect("error when migrate up!");
    });
}

pub fn migration_down() {
    let conn = &mut create_connection();
    let builder = SqliteQueryBuilder;
    let up_migrations = vec![Product::get_down_migration(builder)];

    up_migrations.iter().for_each(|item| {
        let result = diesel::sql_query(item).execute(conn);
        result.expect("error when migrate down!");
    });
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_up_migration() {
        migration_down();
        migrate_up();
    }
}
