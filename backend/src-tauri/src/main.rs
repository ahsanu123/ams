// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod helper;
mod initialization;
mod migration;
mod model;
mod repository;

fn main() {
    let env_var = helper::environment_variable::ENV_VAR.get().unwrap();

    ams_lib::run()
}
