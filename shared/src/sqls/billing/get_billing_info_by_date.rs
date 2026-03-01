use chrono::NaiveDateTime;
use sea_orm::{DatabaseBackend, DbErr, FromQueryResult, Statement};

use crate::repositories::database_connection::get_database_connection;

#[derive(Debug, Clone, FromQueryResult)]
pub struct GetQueryResult {
    pub retrieve_data_id: i64,
    pub price_id: i64,
    pub amount: i64,
    pub is_paid: bool,
    pub date: NaiveDateTime,

    pub customer_id: i64,
    pub customer_name: String,
    pub is_active: bool,
    pub is_admin: bool,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,

    pub bill: f64,
    pub total_amount: i64,

    pub from: NaiveDateTime,
    pub to: NaiveDateTime,
}

const GET_BILLING_INFO_BY_DATE: &str = include_str!("./get_billing_info_by_date.sql");

pub async fn query(
    start_date: NaiveDateTime,
    end_date: NaiveDateTime,
) -> Result<Vec<GetQueryResult>, DbErr> {
    let conn = get_database_connection().await;

    let stmt = Statement::from_sql_and_values(
        DatabaseBackend::Sqlite,
        GET_BILLING_INFO_BY_DATE,
        [start_date.into(), end_date.into()],
    );

    GetQueryResult::find_by_statement(stmt).all(conn).await
}
