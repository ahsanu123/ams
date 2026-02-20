use tokio::sync::Mutex;

// use crate::commands::{
//     customer_money_command::CustomerMoneyCommand, dreg_price_command::DregPriceCommand,
//     make_payment_command::MakePaymentCommand, payment_history_command::PaymentHistoryCommad,
//     production_record_command::ProductionRecordCommand, taking_record_command::TakingRecordCommand,
//     user_management_command::UserManagementCommand,
// };
use std::sync::LazyLock;

// NOTE:
// singleton for service maybe look bad,
// but this application is not connected to multiple client,
// it only run on single computer and only one client at a time,
// so using singleton is enough i think.

// pub static CUSTOMER_MONEY_COMMAND: LazyLock<Mutex<CustomerMoneyCommand>> =
//     LazyLock::new(|| Mutex::new(CustomerMoneyCommand::default()));
//
// pub static DREG_PRICE_COMMAND: LazyLock<Mutex<DregPriceCommand>> =
//     LazyLock::new(|| Mutex::new(DregPriceCommand::default()));
//
// pub static MAKE_PAYMENT_COMMAND: LazyLock<Mutex<MakePaymentCommand>> =
//     LazyLock::new(|| Mutex::new(MakePaymentCommand::default()));
//
// pub static PAYMENT_HISTORY_COMMAND: LazyLock<Mutex<PaymentHistoryCommad>> =
//     LazyLock::new(|| Mutex::new(PaymentHistoryCommad::default()));
//
// pub static PRODUCTION_RECORD_COMMAND: LazyLock<Mutex<ProductionRecordCommand>> =
//     LazyLock::new(|| Mutex::new(ProductionRecordCommand::default()));
//
// pub static TAKING_RECORD_COMMAND: LazyLock<Mutex<TakingRecordCommand>> =
//     LazyLock::new(|| Mutex::new(TakingRecordCommand::default()));
//
// pub static USER_MANAGEMENT_COMMAND: LazyLock<Mutex<UserManagementCommand>> =
//     LazyLock::new(|| Mutex::new(UserManagementCommand::default()));
