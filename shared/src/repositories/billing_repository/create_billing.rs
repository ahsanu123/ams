use anyhow::Result;
use chrono::NaiveDateTime;
use sea_orm::{DatabaseBackend, DbErr, FromQueryResult, Statement};

use crate::repositories::database_connection::get_database_connection;

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

#[derive(Debug, Clone, FromQueryResult)]
pub struct BillingQueryResult {
    pub billing_id: i64,
    pub customer_id: i64,
    pub date: NaiveDateTime,
}

const CREATE_BILLING_SP: &str = include_str!("./create_billing.sql");

pub async fn query(value: CreateQueryResult) -> Result<i64> {
    let conn = get_database_connection().await;

    let stmt = Statement::from_sql_and_values(
        DatabaseBackend::Sqlite,
        CREATE_BILLING_SP,
        [
            value.customer_id.into(),
            value.date.into(),
            value.from.into(),
            value.to.into(),
            value.customer_id.into(),
            value.from.into(),
            value.to.into(),
            value.customer_id.into(),
        ],
    );
    let result = BillingQueryResult::find_by_statement(stmt)
        .one(conn)
        .await?
        .ok_or(DbErr::Custom(
            "fail to get last inserted billing data".into(),
        ))?;

    Ok(result.billing_id)
}
