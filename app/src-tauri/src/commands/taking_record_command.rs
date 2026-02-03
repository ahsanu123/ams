use ams_entity::taking_record_table;
use ams_shared::{
    models::make_payment_page_model::RangePaymentInfo, prelude::*,
    singletons::TAKING_RECORD_COMMAND,
};
use chrono::NaiveDateTime;
use tauri;

#[tauri::command(rename_all = "snake_case")]
pub async fn get_taking_record_by_day(date: NaiveDateTime) -> Vec<taking_record_table::Model> {
    let result = TAKING_RECORD_COMMAND
        .lock()
        .await
        .get_taking_record_by_day(date)
        .await;

    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn upsert_taking_record_by_date(user_id: i32, amount: i32, date: NaiveDateTime) -> i32 {
    let result = TAKING_RECORD_COMMAND
        .lock()
        .await
        .upsert_taking_record_by_date(user_id, amount, date)
        .await;

    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn delete_taking_record_by_id(record_id: i32) -> u64 {
    let result = TAKING_RECORD_COMMAND
        .lock()
        .await
        .delete_taking_record(record_id)
        .await;

    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn add_new_taking_record(user_id: i32, amount: i32) -> i32 {
    let result = TAKING_RECORD_COMMAND
        .lock()
        .await
        .add_new_taking_record(user_id, amount)
        .await;

    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn add_new_taking_record_by_date(user_id: i32, amount: i32, date: NaiveDateTime) -> i32 {
    let result = TAKING_RECORD_COMMAND
        .lock()
        .await
        .add_new_taking_record_by_date(user_id, amount, date)
        .await;

    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_taking_record_by_user_id(user_id: i32) -> Vec<taking_record_table::Model> {
    let result = TAKING_RECORD_COMMAND
        .lock()
        .await
        .get_taking_record_by_user_id(user_id)
        .await;

    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn upsert_taking_record(record: taking_record_table::Model) -> i32 {
    let result = TAKING_RECORD_COMMAND
        .lock()
        .await
        .upsert_taking_record(record.clone())
        .await;
    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_taking_record_by_month(date: NaiveDateTime) -> Vec<taking_record_table::Model> {
    let result = TAKING_RECORD_COMMAND
        .lock()
        .await
        .get_taking_record_by_month(date)
        .await;
    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_taking_record_by_user_id_and_month(
    user_id: i32,
    date: NaiveDateTime,
) -> Vec<taking_record_table::Model> {
    let result = TAKING_RECORD_COMMAND
        .lock()
        .await
        .get_taking_record_by_user_id_and_month(user_id, date)
        .await;
    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_taking_record_by_user_id_and_year(
    user_id: i32,
    date: NaiveDateTime,
) -> Vec<taking_record_table::Model> {
    let result = TAKING_RECORD_COMMAND
        .lock()
        .await
        .get_taking_record_by_user_id_and_year(user_id, date)
        .await;
    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_taking_record_by_user_id_and_month_range(
    user_id: i32,
    from: NaiveDateTime,
    to: NaiveDateTime,
) -> RangePaymentInfo {
    let result = TAKING_RECORD_COMMAND
        .lock()
        .await
        .get_taking_record_by_user_id_and_month_range(user_id, from, to)
        .await;
    result
}
