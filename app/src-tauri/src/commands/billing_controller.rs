use ams_shared::{
    controllers::{
        billing_controller::{
            BillingControllerErr, BillingControllerTrait, BillingGetByProps, BillingInfoGetAllProps,
        },
        BILLING_CONTROLLER,
    },
    models::billing::{
        billing_info::{BillingInfo, BillingInfoWithBalance},
        BillingCreate,
    },
};

#[tauri::command(rename_all = "snake_case")]
pub async fn get_all_billing_info(
    query: BillingInfoGetAllProps,
) -> Result<Vec<BillingInfo>, BillingControllerErr> {
    let result = BILLING_CONTROLLER
        .lock()
        .await
        .get_all_billing_info(query)
        .await;

    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn billing_create(request: BillingCreate) -> Result<BillingInfo, BillingControllerErr> {
    let result = BILLING_CONTROLLER.lock().await.create(request).await;

    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn billing_get_by(
    query: BillingGetByProps,
) -> Result<Vec<BillingInfoWithBalance>, BillingControllerErr> {
    let result = BILLING_CONTROLLER.lock().await.get_by(query).await;

    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_all_billing() -> Result<Vec<BillingInfoWithBalance>, BillingControllerErr> {
    let result = BILLING_CONTROLLER.lock().await.get_all_billing().await;

    result
}
