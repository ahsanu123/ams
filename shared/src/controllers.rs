use crate::controllers::{
    balance_controller::BalanceController, billing_controller::BillingController,
    customer_controller::CustomerController, price_controller::PriceController,
    retrieve_data_controller::RetrieveDataController,
};
use std::sync::LazyLock;
use tokio::sync::Mutex;

pub mod balance_controller;
pub mod billing_controller;
pub mod customer_controller;
pub mod data_record_controller;
pub mod price_controller;
pub mod retrieve_data_controller;

pub static RETRIEVE_DATA_CONTROLLER: LazyLock<Mutex<RetrieveDataController>> =
    LazyLock::new(|| Mutex::new(RetrieveDataController));

pub static CUSTOMER_CONTROLLER: LazyLock<Mutex<CustomerController>> =
    LazyLock::new(|| Mutex::new(CustomerController));

pub static BALANCE_CONTROLLER: LazyLock<Mutex<BalanceController>> =
    LazyLock::new(|| Mutex::new(BalanceController));

pub static BILLING_CONTROLLER: LazyLock<Mutex<BillingController>> =
    LazyLock::new(|| Mutex::new(BillingController));

pub static PRICE_CONTROLLER: LazyLock<Mutex<PriceController>> =
    LazyLock::new(|| Mutex::new(PriceController));
