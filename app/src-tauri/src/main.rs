#![allow(warnings)]
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod helper;
mod migration;
mod model;
mod repository;

fn main() {
    ams_lib::run()
}
