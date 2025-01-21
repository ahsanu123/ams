// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod controller;
mod model;
mod repository;

fn main() {
    let conn = controller::UserController {};
    let user = model::User::default();
    let repo_user = repository::UserRepository { user };
    let m = model::Shop::default();
    let f = repository::ProductRepository {};
    ams_lib::run()
}
