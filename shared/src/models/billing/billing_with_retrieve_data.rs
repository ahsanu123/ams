use crate::models::{
    customer::Customer,
    retrieve_data::retrieve_data_with_customer_and_price::RetrieveDataWithCustomerAndPrice,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Default, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct BillingWithRetrieveData {
    pub billing_id: i64,
    pub customer_id: i64,

    #[ts(type = "Date")]
    pub date: NaiveDateTime,

    pub customer: Customer,
    #[ts(type = "Date")]
    pub from: NaiveDateTime,
    #[ts(type = "Date")]
    pub to: NaiveDateTime,
    pub bill: f64,
    pub amount: i64,

    pub retrieves_data: Vec<RetrieveDataWithCustomerAndPrice>,
}
