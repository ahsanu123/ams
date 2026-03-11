use ams_shared::{
    controllers::{
        price_controller::{PriceControllerErr, PriceControllerTrait},
        PRICE_CONTROLLER,
    },
    models::price::Price,
};

#[tauri::command(rename_all = "snake_case")]
pub async fn price_get_latest() -> Result<Price, PriceControllerErr> {
    let result = PRICE_CONTROLLER.lock().await.get_latest_dreg_price().await;

    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn price_update(new_price: f32) -> Result<i64, PriceControllerErr> {
    let result = PRICE_CONTROLLER
        .lock()
        .await
        .update_dreg_price(new_price)
        .await;

    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn price_get_all() -> Result<Vec<Price>, PriceControllerErr> {
    let result = PRICE_CONTROLLER.lock().await.get_all_dreg_price().await;

    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn price_delete(price_id: i64) -> Result<u64, PriceControllerErr> {
    let result = PRICE_CONTROLLER.lock().await.delete_by_id(price_id).await;

    result
}
