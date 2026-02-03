use ams_shared::{
    models::make_payment_page_model::MakePaymentPageModel, prelude::*,
    singletons::MAKE_PAYMENT_COMMAND,
};
use chrono::NaiveDateTime;

#[tauri::command(rename_all = "snake_case")]
pub async fn payment_page_get_page_model(
    user_id: i32,
    date: NaiveDateTime,
) -> MakePaymentPageModel {
    let result = MAKE_PAYMENT_COMMAND
        .lock()
        .await
        .get_page_model(user_id, date)
        .await
        .unwrap();

    result
}

#[tauri::command(rename_all = "snake_case")]
pub async fn payment_page_make_payment(user_id: i32, date: NaiveDateTime) -> MakePaymentPageModel {
    let result = MAKE_PAYMENT_COMMAND
        .lock()
        .await
        .make_payment(user_id, date)
        .await
        .unwrap();

    result
}
