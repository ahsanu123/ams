use std::sync::LazyLock;

use tokio::sync::Mutex;

use crate::repositories::{
    balance_billing_repository::BalanceBillingRepository, balance_repository::BalanceRepository,
    billing_repository::BillingRepository,
    billing_retrieve_data_repository::BillingRetrieveDataRepository,
    customer_repository::CustomerRepository, data_record_repository::DataRecordRepository,
    price_repositories::PriceRepository,
};

pub mod balance_billing_repository;
pub mod balance_repository;
pub mod base_repository_trait;
pub mod billing_repository;
pub mod billing_retrieve_data_repository;
pub mod customer_repository;
pub mod data_record_repository;
pub mod generic_crud_repository;
pub mod price_repositories;
pub mod retrieve_data_repository;

pub(crate) mod database_connection;

// NOTE:
// this application is intended for local use only,
// no more than 5 user at same time is expected
// so lifetime for repositories is not worth it to create,
// instead just use singleton is enough.

pub static BALANCE_BILLING_REPO: LazyLock<Mutex<BalanceBillingRepository>> =
    LazyLock::new(|| Mutex::new(BalanceBillingRepository));

pub static BALANCE_REPO: LazyLock<Mutex<BalanceRepository>> =
    LazyLock::new(|| Mutex::new(BalanceRepository));

pub static BILLING_REPO: LazyLock<Mutex<BillingRepository>> =
    LazyLock::new(|| Mutex::new(BillingRepository));

pub static BILLING_RETRIEVE_DATA_REPO: LazyLock<Mutex<BillingRetrieveDataRepository>> =
    LazyLock::new(|| Mutex::new(BillingRetrieveDataRepository));

pub static CUSTOMER_REPO: LazyLock<Mutex<CustomerRepository>> =
    LazyLock::new(|| Mutex::new(CustomerRepository));

pub static DATA_RECORD_REPO: LazyLock<Mutex<DataRecordRepository>> =
    LazyLock::new(|| Mutex::new(DataRecordRepository));

pub static PRICE_REPO: LazyLock<Mutex<PriceRepository>> =
    LazyLock::new(|| Mutex::new(PriceRepository));

pub static RETRIEVE_DATA_REPO: LazyLock<Mutex<BillingRetrieveDataRepository>> =
    LazyLock::new(|| Mutex::new(BillingRetrieveDataRepository));
