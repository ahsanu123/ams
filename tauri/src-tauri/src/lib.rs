// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod commands;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    //let http = tauri_invoke_http::Invoke::new(if cfg!(feature = "custom-protocol") {
    //    ["tauri://localhost"]
    //} else {
    //    ["http://localhost:8080"]
    //});
    //
    //tauri::Builder::default()
    //    .invoke_system(http.initialization_script())
    //    .setup(move |app| {
    //        http.start(app.handle());
    //        Ok(())
    //    })
    //    .run(tauri::generate_context!())
    //    .expect("error while running tauri application")

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![commands::greet, commands::hello])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
