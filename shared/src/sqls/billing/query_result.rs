use chrono::NaiveDateTime;
use sea_orm::FromQueryResult;

#[derive(Debug, Clone, Copy, FromQueryResult)]
pub struct GetQueryResult {
    pub billing_id: i64,
    pub customer_id: i64,
    pub date: NaiveDateTime,

    pub from: NaiveDateTime,
    pub to: NaiveDateTime,
    pub amount: i64,
    pub bill: f64,
}

#[derive(Debug, Clone, Copy, FromQueryResult)]
pub struct CreateQueryResult {
    pub customer_id: i64,
    pub date: NaiveDateTime,

    pub from: NaiveDateTime,
    pub to: NaiveDateTime,
}
