// pub mod customer_money_command;
// pub mod dreg_price_command;
// pub mod make_payment_command;
// pub mod payment_history_command;
// pub mod production_record_command;
// pub mod taking_record_command;
// pub mod user_management_command;

use std::sync::LazyLock;

use tokio::sync::Mutex;

use crate::controllers::retrieve_data_controller::RetrieveDataController;

pub mod balance_controller;
pub mod billing_controller;
pub mod customer_controller;
pub mod data_record_controller;
pub mod price_controller;
pub mod retrieve_data_controller;

pub static RETRIEVE_DATA_CONTROLLER: LazyLock<Mutex<RetrieveDataController>> =
    LazyLock::new(|| Mutex::new(RetrieveDataController));
