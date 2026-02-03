// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

pub mod builder;
pub mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    std::thread::spawn(ams_api::start_server_blocking);
    builder::build_and_run();
}
