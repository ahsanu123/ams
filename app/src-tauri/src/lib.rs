use ams_shared::init_environment_variable;

pub mod builder;
pub mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init_environment_variable();
    builder::build_and_run();
}
