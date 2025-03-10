// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

pub mod commands;
pub mod helper;
pub mod migration;
pub mod model;
pub mod query;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![commands::greet, commands::hello])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
