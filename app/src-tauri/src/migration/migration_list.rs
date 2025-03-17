use crate::helper::environment_variable::ENV_VAR;
use crate::helper::sql_connection_helper::create_connection;
use crate::migration::migration_trait::Migrationable;
use crate::model::admin_user_seeds::{AdminUserSeeds, SeedTrait};
use crate::model::database_metadata::Metadata;
use crate::model::user_seeds::UserSeed;
use crate::model::{product::Product, user::User};
use diesel::RunQueryDsl;
use sea_query::SqliteQueryBuilder;

pub fn migrate_up() {
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

pub fn create_database_metadata() {
    let conn = &mut create_connection();
    let create_db_metadata = Metadata::get_up_migration(SqliteQueryBuilder);
    let result = diesel::sql_query(create_db_metadata).execute(conn);

    result.expect("Error When trying to migrate database metadata!!!");
}

pub fn seed_database() {
    let conn = &mut create_connection();

    AdminUserSeeds::default()
        .seed_db(conn)
        .expect("cant seed admin");

    UserSeed::default().seed_db(conn).expect("cant seed user");
}

pub fn setup_database() {
    let latest_version = Metadata::get_latest_version();

    if ENV_VAR.ams_database_version > latest_version {
        migration_down();

        create_database_metadata();

        migrate_up();
        seed_database();

        Metadata::add_migration_stamp();
    }
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

    #[test]
    fn test_setup_database() {
        setup_database();
    }

    #[test]
    fn test_seed_database() {
        seed_database();
    }
}
