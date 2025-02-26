#![allow(warnings)]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use migration::migration_list;
mod helper;
mod migration;
mod model;
mod repository;

fn main() {
    // TODO: make best approach to drop and re-create table when
    //       database version increased, currently there is not metadata
    //       for database version etc.

    // migration_list::migration_down();
    migration_list::migrate_up();

    ams_lib::run()
}
