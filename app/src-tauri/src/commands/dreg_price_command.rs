use ams_entity::price_history_table;
use ams_shared::{prelude::*, singletons::DREG_PRICE_COMMAND};

#[tauri::command(rename_all = "snake_case")]
pub async fn get_latest_dreg_price() -> price_history_table::Model {
    let result = DREG_PRICE_COMMAND
        .lock()
        .await
        .get_latest_dreg_price()
        .await
        .unwrap();

    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn update_dreg_price(new_price: i64) -> price_history_table::Model {
    let result = DREG_PRICE_COMMAND
        .lock()
        .await
        .update_dreg_price(new_price)
        .await;

    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_all_dreg_price() -> Vec<price_history_table::Model> {
    let result = DREG_PRICE_COMMAND.lock().await.get_all_dreg_price().await;

    result
}
