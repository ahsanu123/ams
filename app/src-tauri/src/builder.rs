use crate::commands::{
    customer_command, dreg_price_command, make_payment_page_command, payment_history_command,
    taking_record_command, user_management_command,
};
use ams_shared::helper::{environment_variable::EnvironmentVariable, ENV_VAR};
use dotenvy::from_path;
use std::{env, path};

pub fn build_and_run() {
    tauri::Builder::default()
        .setup(|_| {
            dotenvy::dotenv().ok();

            // NOTE:
            // currently to read database file, db file must be
            // in same directory of exe, i have try to use
            // embedded resource but it not extracting any file
            // (on linux it refer to /usr/lib/progname)
            // and /usr/lib/ is need admin right,
            // i dont want run program with admin right,
            // so will use this method until i found better way
            // to handle this.

            // let env_path = app.path().resolve(".env", BaseDirectory::Resource).unwrap();
            // log::info!("{}", env_path.clone().to_string_lossy());
            //
            // let sqlite_path = app
            //     .path()
            //     .resolve("ams.sqlite", BaseDirectory::Resource)
            //     .unwrap();
            //
            // let sqlite_conn = format!("sqlite://{}?mode=rwc", sqlite_path.to_string_lossy());

            let current_dir = env::current_dir().unwrap();
            let env_path = path::PathBuf::from(format!("{}/.env", current_dir.display()));
            log::info!("{}", path::absolute(&env_path).unwrap().to_string_lossy());
            let abs_env_path = path::absolute(&env_path).unwrap();
            let _ = from_path(abs_env_path);

            let production_mode = env::args().any(|arg| arg.contains("--production-mode"));

            let sqlite_path: path::PathBuf = if production_mode {
                path::PathBuf::from(format!("{}/ams-prod.sqlite", current_dir.display()))
            } else {
                path::PathBuf::from(format!("{}/ams.sqlite", current_dir.display()))
            };

            log::info!(
                "sqlite://{}?mode=rwc",
                path::absolute(&sqlite_path).unwrap().to_string_lossy()
            );
            let abs_sqlite_path = std::path::absolute(&sqlite_path)
                .unwrap()
                .into_os_string()
                .into_string()
                .unwrap();
            let abs_sqlite_path = format!("sqlite://{}?mode=rwc", abs_sqlite_path);

            ENV_VAR
                .set(EnvironmentVariable {
                    sqlite_connection_string: abs_sqlite_path,
                })
                .unwrap();

            Ok(())
        })
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
