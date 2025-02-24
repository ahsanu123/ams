// Prevents additional console window on Windows in release, DO NOT REMOVE!!

#![allow(warnings)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod helper;
// mod initialization;
mod migration;
mod model;
mod repository;

fn main() {
    ams_lib::run()
}
