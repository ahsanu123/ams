use chrono::NaiveDateTime;

use crate::models::{customer::Customer, price::Price};

pub struct RetrieveData {
    pub retrieve_data_id: i64,
    pub customer_id: i64,
    pub price_id: i64,
    pub amount: i64,
    pub date: NaiveDateTime,
    pub is_paid: bool,
    pub customer: Customer,
    pub price: Price,
}
