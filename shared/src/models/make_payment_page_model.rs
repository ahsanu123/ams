use ams_entity::{price_history_table, taking_record_table, user_table};
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DetailInformation {
    pub total_bill: i64,
    pub total_amount: i64,
    pub paid_bill: i64,
    pub paid_amount: i64,
    pub unpaid_bill: i64,
    pub unpaid_amount: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TakingRecordWithPrice {
    pub taking_record: taking_record_table::Model,
    pub price: price_history_table::Model,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MakePaymentPageModel {
    pub taking_records: Vec<TakingRecordWithPrice>,
    pub detail_information: DetailInformation,
    pub customers: Vec<user_table::Model>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RangePaymentInfo {
    pub from: NaiveDateTime,
    pub to: NaiveDateTime,
    pub record_with_price: Vec<TakingRecordWithPrice>,
    pub detail_information: DetailInformation,
    pub customer: user_table::Model,
}
