use crate::repositories::database_connection::get_database_connection;
use anyhow::Result;
use chrono::NaiveDateTime;
use sea_orm::{DatabaseBackend, DbErr, FromQueryResult, Statement};

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

const GET_BY_YEAR_SP: &str = include_str!("./get_by_year.sql");

pub async fn query(
    start_year: NaiveDateTime,
    end_year: NaiveDateTime,
) -> Result<Vec<GetQueryResult>, DbErr> {
    let conn = get_database_connection().await;

    let stmt = Statement::from_sql_and_values(
        DatabaseBackend::Sqlite,
        GET_BY_YEAR_SP,
        [start_year.into(), end_year.into()],
    );

    GetQueryResult::find_by_statement(stmt).all(conn).await
}
