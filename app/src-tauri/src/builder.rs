use crate::commands::{
    customer_command, dreg_price_command, make_payment_page_command, payment_history_command,
    taking_record_command, user_management_command,
};
use std::env;

pub fn build_and_run() {
    tauri::Builder::default()
        .setup(|_| Ok(()))
        // .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // customer_command
            customer_command::delete_customer,
            customer_command::get_all_user_money_history,
            customer_command::add_money,
            // dreg_price_command
            dreg_price_command::get_latest_dreg_price,
            dreg_price_command::update_dreg_price,
            dreg_price_command::get_all_dreg_price,
            // make_payment_page_command
            make_payment_page_command::payment_page_get_page_model,
            make_payment_page_command::payment_page_make_payment,
            // payment_history_command
            payment_history_command::get_payment_record_by_user_id,
            payment_history_command::get_month_summary,
            payment_history_command::get_payment_record_by_user_id_and_month,
            payment_history_command::update_payment_record,
            payment_history_command::update_bulk_payment_record,
            // taking_record_command
            taking_record_command::get_taking_record_by_day,
            taking_record_command::upsert_taking_record_by_date,
            taking_record_command::delete_taking_record_by_id,
            taking_record_command::add_new_taking_record,
            taking_record_command::add_new_taking_record_by_date,
            taking_record_command::get_taking_record_by_user_id,
            taking_record_command::upsert_taking_record,
            taking_record_command::get_taking_record_by_month,
            taking_record_command::get_taking_record_by_user_id_and_month_range,
            taking_record_command::get_taking_record_by_user_id_and_month,
            taking_record_command::get_taking_record_by_user_id_and_year,
            // user_management_command
            user_management_command::create_new_user,
            user_management_command::insert_new_user,
            user_management_command::get_all_user,
            user_management_command::get_all_active_user,
            user_management_command::upsert_user,
            user_management_command::get_by_user_id,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
