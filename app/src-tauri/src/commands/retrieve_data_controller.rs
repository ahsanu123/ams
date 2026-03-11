#![allow(dead_code)]
use ams_shared::{
    controllers::{
        retrieve_data_controller::{
            RetrieveDataControllerErr, RetrieveDataControllerTrait, RetrieveDataGetAllProps,
        },
        RETRIEVE_DATA_CONTROLLER,
    },
    models::retrieve_data::{
        retrieve_data_create_or_update::{
            RetrieveDataCreate, RetrieveDataCreateOrUpdate, RetrieveDataCreateWithDate,
        },
        retrieve_data_with_customer_and_price::RetrieveDataWithCustomerAndPrice,
    },
};

#[tauri::command(rename_all = "snake_case")]
pub async fn retrieve_data_create(
    request: RetrieveDataCreate,
) -> Result<i64, RetrieveDataControllerErr> {
    let result = RETRIEVE_DATA_CONTROLLER.lock().await.create(request).await;

    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn retrieve_data_create_with_date(
    request: RetrieveDataCreateWithDate,
) -> Result<i64, RetrieveDataControllerErr> {
    let result = RETRIEVE_DATA_CONTROLLER
        .lock()
        .await
        .create_wd(request)
        .await;

    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn retrieve_data_update(
    request: RetrieveDataCreateOrUpdate,
) -> Result<Option<RetrieveDataWithCustomerAndPrice>, RetrieveDataControllerErr> {
    let result = RETRIEVE_DATA_CONTROLLER.lock().await.update(request).await;

    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn retrieve_data_get_all(
    request: RetrieveDataGetAllProps,
) -> Result<Vec<RetrieveDataWithCustomerAndPrice>, RetrieveDataControllerErr> {
    let result = RETRIEVE_DATA_CONTROLLER.lock().await.get_all(request).await;

    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn retrieve_data_delete(retrieve_data_id: i64) -> Result<u64, RetrieveDataControllerErr> {
    let result = RETRIEVE_DATA_CONTROLLER
        .lock()
        .await
        .delete(retrieve_data_id)
        .await;

    result
}
