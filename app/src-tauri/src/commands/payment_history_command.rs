use ams_entity::payment_history_table;
use ams_shared::{prelude::*, singletons::PAYMENT_HISTORY_COMMAND};
use chrono::NaiveDateTime;
use tauri;

#[tauri::command(rename_all = "snake_case")]
pub async fn get_payment_record_by_user_id(user_id: i32) -> Vec<payment_history_table::Model> {
    let result = PAYMENT_HISTORY_COMMAND
        .lock()
        .await
        .get_payment_record_by_user_id(user_id)
        .await;

    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_month_summary(date: NaiveDateTime) -> Vec<payment_history_table::Model> {
    let result = PAYMENT_HISTORY_COMMAND
        .lock()
        .await
        .get_month_summary(date)
        .await;
    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_payment_record_by_user_id_and_month(
    user_id: i32,
    date: NaiveDateTime,
) -> Vec<payment_history_table::Model> {
    let result = PAYMENT_HISTORY_COMMAND
        .lock()
        .await
        .get_payment_record_by_user_id_and_month(user_id, date)
        .await;
    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn update_payment_record(
    record: payment_history_table::Model,
) -> payment_history_table::Model {
    let result = PAYMENT_HISTORY_COMMAND
        .lock()
        .await
        .update_payment_record(record.clone())
        .await;
    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn update_bulk_payment_record(
    records: Vec<payment_history_table::Model>,
    paid: bool,
) -> u64 {
    let result = PAYMENT_HISTORY_COMMAND
        .lock()
        .await
        .update_bulk_payment_record(records.clone(), paid)
        .await;

    result
}
