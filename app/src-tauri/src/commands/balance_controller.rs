use ams_shared::{
    controllers::{
        balance_controller::{BalanceControllerErr, BalanceControllerTrait, BalanceGetAllProps},
        BALANCE_CONTROLLER,
    },
    models::balance::{BalanceCreateOrUpdateWithoutChangedValue, BalanceWithCustomer},
};

#[tauri::command(rename_all = "snake_case")]
pub async fn balance_add_balance(
    request: BalanceCreateOrUpdateWithoutChangedValue,
) -> Result<i64, BalanceControllerErr> {
    let result = BALANCE_CONTROLLER.lock().await.add_balance(request).await;

    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn balance_get_latest_by_customer_id(
    customer_id: i64,
) -> Result<BalanceWithCustomer, BalanceControllerErr> {
    let result = BALANCE_CONTROLLER
        .lock()
        .await
        .get_latest(customer_id)
        .await;

    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn balance_get_all(
    query: BalanceGetAllProps,
) -> Result<Vec<BalanceWithCustomer>, BalanceControllerErr> {
    let result = BALANCE_CONTROLLER.lock().await.get_all(query).await;
    result
}
