use chrono::NaiveDateTime;

pub struct BillingRetrieveData {
    pub billing_retrieve_data_id: i64,
    pub billing_id: i64,
    pub retrieve_data_id: i64,
    pub from: NaiveDateTime,
    pub to: NaiveDateTime,
    pub bill: f64,
    pub amount: i64,
}
