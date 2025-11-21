use ams_shared::{
    commands::make_payment_command::{MakePaymentCommand, MakePaymentCommandTrait},
    models::make_payment_page_model::MakePaymentPageModel,
};
use chrono::NaiveDateTime;

#[tauri::command]
pub async fn payment_page_get_page_model(
    user_id: i32,
    date: NaiveDateTime,
) -> MakePaymentPageModel {
    let result = MakePaymentCommand::get_page_model(user_id, date)
        .await
        .unwrap();

    result
}

#[tauri::command]
pub async fn payment_page_make_payment(user_id: i32, date: NaiveDateTime) -> MakePaymentPageModel {
    let result = MakePaymentCommand::make_payment(user_id, date)
        .await
        .unwrap();

    result
}
