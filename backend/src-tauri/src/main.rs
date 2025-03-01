#![allow(warnings)]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ams_lib::model::database_metadata::DatabaseMetadata;
use migration::{migration_list, table::get_db_version};
mod helper;
mod migration;
mod model;
mod repository;

fn main() {
    // TODO: make best approach to drop and re-create table when
    //       database version increased, currently there is not metadata
    //       for database version etc.
    //       add fresh update database (delete all table including database metadata, and re
    //       seeding it)

    let db_version = 1;
    migration_list::migrate_database_metadata();

    // migration_list::migration_down();
    migration_list::migrate_up();

    ams_lib::run()
}
