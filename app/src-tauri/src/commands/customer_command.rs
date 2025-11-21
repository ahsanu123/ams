use ams_entity::{money_history_table, user_table};
use ams_shared::commands::customer_money_command::{
    CustomerMoneyCommand, CustomerMoneyCommandTrait,
};
use tauri;

#[tauri::command(rename_all = "snake_case")]
pub async fn add_money(user_id: i64, amount: i64) -> user_table::Model {
    let result = CustomerMoneyCommand::add_money(user_id, amount)
        .await
        .unwrap();
    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_all_user_money_history(user_id: i64) -> Vec<money_history_table::Model> {
    let result = CustomerMoneyCommand::get_all_user_money_history(user_id).await;

    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn delete_customer(user_id: i32) -> u64 {
    let result = CustomerMoneyCommand::delete_user(user_id as i32)
        .await
        .unwrap();

    result
}
