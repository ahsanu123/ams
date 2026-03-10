use ams_shared::{
    controllers::{
        customer_controller::{CustomerControllerErr, CustomerControllerTrait, CustomerGetAllProp},
        CUSTOMER_CONTROLLER,
    },
    models::customer::{Customer, CustomerUpdate},
};

#[tauri::command(rename_all = "snake_case")]
pub async fn customer_management_create(
    customer_name: String,
) -> Result<i64, CustomerControllerErr> {
    let result = CUSTOMER_CONTROLLER
        .lock()
        .await
        .create_new_customer(customer_name)
        .await;
    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn customer_management_update(
    request: CustomerUpdate,
) -> Result<i64, CustomerControllerErr> {
    let result = CUSTOMER_CONTROLLER.lock().await.update(request).await;

    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn customer_management_get_first_customer() -> Result<Customer, CustomerControllerErr> {
    let result = CUSTOMER_CONTROLLER.lock().await.get_first_customer().await;

    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn customer_management_get_all(
    query: CustomerGetAllProp,
) -> Result<Vec<Customer>, CustomerControllerErr> {
    let result = CUSTOMER_CONTROLLER.lock().await.get_all(query).await;

    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn customer_management_get_by_id(
    customer_id: i64,
) -> Result<Option<Customer>, CustomerControllerErr> {
    let result = CUSTOMER_CONTROLLER
        .lock()
        .await
        .get_by_customer_id(customer_id)
        .await;

    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn customer_management_delete(customer_id: i64) -> Result<u64, CustomerControllerErr> {
    let result = CUSTOMER_CONTROLLER.lock().await.delete(customer_id).await;

    result
}
