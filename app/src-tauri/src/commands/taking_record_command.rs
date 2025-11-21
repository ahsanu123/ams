use ams_entity::taking_record_table;
use ams_shared::commands::taking_record_command::{TakingRecordCommand, TakingRecordCommandTrait};
use chrono::NaiveDateTime;
use tauri;

#[tauri::command]
pub async fn get_taking_record_by_day(date: NaiveDateTime) -> Vec<taking_record_table::Model> {
    let result = TakingRecordCommand::get_taking_record_by_day(date).await;
    result
}

#[tauri::command]
pub async fn upsert_taking_record_by_date(user_id: i32, amount: i32, date: NaiveDateTime) -> i32 {
    let result = TakingRecordCommand::upsert_taking_record_by_date(user_id, amount, date).await;
    result
}

#[tauri::command]
pub async fn delete_taking_record_by_id(record_id: i32) -> u64 {
    let result = TakingRecordCommand::delete_taking_record(record_id).await;
    result
}

#[tauri::command]
pub async fn add_new_taking_record(user_id: i32, amount: i32) -> i32 {
    let result = TakingRecordCommand::add_new_taking_record(user_id, amount).await;
    result
}

#[tauri::command]
pub async fn add_new_taking_record_by_date(user_id: i32, amount: i32, date: NaiveDateTime) -> i32 {
    let result = TakingRecordCommand::add_new_taking_record_by_date(user_id, amount, date).await;
    result
}

#[tauri::command]
pub async fn get_taking_record_by_user_id(user_id: i32) -> Vec<taking_record_table::Model> {
    let result = TakingRecordCommand::get_taking_record_by_user_id(user_id).await;
    result
}

#[tauri::command]
pub async fn upsert_taking_record(record: taking_record_table::Model) -> i32 {
    let result = TakingRecordCommand::upsert_taking_record(record.clone()).await;
    result
}

#[tauri::command]
pub async fn get_taking_record_by_month(date: NaiveDateTime) -> Vec<taking_record_table::Model> {
    let result = TakingRecordCommand::get_taking_record_by_month(date).await;
    result
}

#[tauri::command]
pub async fn get_taking_record_by_user_id_and_month(
    user_id: i32,
    date: NaiveDateTime,
) -> Vec<taking_record_table::Model> {
    let result = TakingRecordCommand::get_taking_record_by_user_id_and_month(user_id, date).await;
    result
}

#[tauri::command]
pub async fn get_taking_record_by_user_id_and_year(
    user_id: i32,
    date: NaiveDateTime,
) -> Vec<taking_record_table::Model> {
    let result = TakingRecordCommand::get_taking_record_by_user_id_and_year(user_id, date).await;
    result
}
