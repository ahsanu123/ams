use crate::commands::{
    balance_controller, billing_controller, customer_management_controller, price_controller,
    retrieve_data_controller,
};
use std::env;

pub fn build_and_run() {
    tauri::Builder::default()
        .setup(|_| {
            std::thread::spawn(ams_api::start_server_blocking);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // price_command
            price_controller::price_get_latest,
            price_controller::price_update,
            price_controller::price_get_all,
            price_controller::price_delete,
            // customer_management_command
            customer_management_controller::customer_management_create,
            customer_management_controller::customer_management_update,
            customer_management_controller::customer_management_get_first_customer,
            customer_management_controller::customer_management_get_all,
            customer_management_controller::customer_management_get_by_id,
            customer_management_controller::customer_management_delete,
            // billing_command
            billing_controller::get_all_billing_info,
            billing_controller::billing_create,
            billing_controller::billing_get_by,
            billing_controller::get_all_billing,
            // balance_command
            balance_controller::balance_add_balance,
            balance_controller::balance_get_latest_by_customer_id,
            balance_controller::balance_get_all,
            // retrieve_data_command
            retrieve_data_controller::retrieve_data_create,
            retrieve_data_controller::retrieve_data_create_with_date,
            retrieve_data_controller::retrieve_data_update,
            retrieve_data_controller::retrieve_data_get_all,
            retrieve_data_controller::retrieve_data_delete,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
