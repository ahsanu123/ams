use ams_entity::user_table;
use ams_shared::{prelude::*, singletons::USER_MANAGEMENT_COMMAND};
use tauri;

#[tauri::command(rename_all = "snake_case")]
pub async fn create_new_user(username: String) -> i32 {
    let result = USER_MANAGEMENT_COMMAND
        .lock()
        .await
        .create_new_user(username)
        .await;

    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn insert_new_user(new_user: user_table::Model) -> i32 {
    let result = USER_MANAGEMENT_COMMAND
        .lock()
        .await
        .insert_new_user(new_user.clone())
        .await;

    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_all_user() -> Vec<user_table::Model> {
    let result = USER_MANAGEMENT_COMMAND.lock().await.get_all_user().await;

    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_all_active_user() -> Vec<user_table::Model> {
    let result = USER_MANAGEMENT_COMMAND
        .lock()
        .await
        .get_all_active_user()
        .await;

    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn upsert_user(user: user_table::Model) -> i32 {
    let result = USER_MANAGEMENT_COMMAND
        .lock()
        .await
        .upsert_user(user.clone())
        .await;

    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_by_user_id(id: i32) -> Option<user_table::Model> {
    let result = USER_MANAGEMENT_COMMAND
        .lock()
        .await
        .get_by_user_id(id)
        .await;

    result
}
