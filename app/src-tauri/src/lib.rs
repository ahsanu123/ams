// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

pub mod authentication;
pub mod commands;
pub mod helper;
pub mod migration;
pub mod model;
pub mod query;
pub mod response;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // commands::user_command::greet,
            // commands::user_command::hello
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
