pub(crate) mod controllers;
pub(crate) mod repositories;
pub(crate) mod sqls;
pub(crate) mod utilities;

use std::env;

// pub use commands::customer_money_command::CustomerMoneyCommandTrait;
// pub use commands::dreg_price_command::DregPriceCommandTrait;
// pub use commands::make_payment_command::MakePaymentCommandTrait;
// pub use commands::payment_history_command::PaymentHistoryCommandTrait;
// pub use commands::production_record_command::ProductionRecordCommandTrait;
// pub use commands::taking_record_command::TakingRecordCommandTrait;
// pub use commands::user_management_command::UserManagementCommandTrait;

use crate::helper::ENV_VAR;
use crate::helper::environment_variable::EnvironmentVariable;

pub mod helper;
pub mod models;
pub mod page_models;
pub mod singletons;

const CONN_STRING_PATH: &str = "sqlite_connection_string";
const STATIC_FILE_PATH: &str = "static_file_path";

pub fn init_environment_variable() {
    let env_path = dotenvy::dotenv().unwrap_or_else(|_| {
        panic!(
            "loading .env failed, cwd: {:?}",
            env::current_dir().unwrap()
        )
    });

    let sqlite_connection_string = std::env::var(CONN_STRING_PATH).unwrap_or_else(|_| {
        panic!(
            "{} is not found, in {}",
            CONN_STRING_PATH,
            env_path.to_string_lossy()
        )
    });

    let static_file_path = std::env::var(STATIC_FILE_PATH).unwrap_or_else(|_| {
        panic!(
            "{} is not found, in {}",
            STATIC_FILE_PATH,
            env_path.to_string_lossy()
        )
    });

    ENV_VAR
        .set(EnvironmentVariable {
            sqlite_connection_string,
            static_file_path,
        })
        .unwrap();
}

pub mod prelude {
    pub use super::*;
}

#[cfg(test)]
mod ams_shared_test {
    use crate::init_environment_variable;

    #[test]
    fn test_environment_variable() {
        init_environment_variable();
    }
}
