use ams_entity::price_history_table;
use ams_shared::commands::dreg_price_command::{DregPriceCommand, DregPriceCommandTrait};

#[tauri::command]
pub async fn get_latest_dreg_price() -> price_history_table::Model {
    let result = DregPriceCommand::get_latest_dreg_price().await.unwrap();
    result
}

#[tauri::command]
pub async fn update_dreg_price(new_price: i64) -> price_history_table::Model {
    let result = DregPriceCommand::update_dreg_price(new_price).await;
    result
}

#[tauri::command]
pub async fn get_all_dreg_price() -> Vec<price_history_table::Model> {
    let result = DregPriceCommand::get_all_dreg_price().await;
    result
}
