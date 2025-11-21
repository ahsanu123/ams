use ams_entity::user_table;
use ams_shared::commands::user_management_command::{
    UserManagementCommand, UserManagementCommandTrait,
};
use tauri;

#[tauri::command(rename_all = "snake_case")]
pub async fn create_new_user(username: String) -> i32 {
    let result = UserManagementCommand::create_new_user(username).await;
    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn insert_new_user(new_user: user_table::Model) -> i32 {
    let result = UserManagementCommand::insert_new_user(new_user.clone()).await;
    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_all_user() -> Vec<user_table::Model> {
    let result = UserManagementCommand::get_all_user().await;
    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_all_active_user() -> Vec<user_table::Model> {
    let result = UserManagementCommand::get_all_active_user().await;
    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn upsert_user(user: user_table::Model) -> i32 {
    let result = UserManagementCommand::upsert_user(user.clone()).await;
    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_by_user_id(id: i32) -> Option<user_table::Model> {
    let result = UserManagementCommand::get_by_user_id(id).await;
    result
}
