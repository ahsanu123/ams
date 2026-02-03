use ams_shared::init_environment_variable;

pub mod builder;
pub mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init_environment_variable();
    // std::thread::spawn(ams_api::start_server_blocking);
    builder::build_and_run();
}
