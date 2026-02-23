use chrono::NaiveDateTime;
use sea_orm::{ConnectionTrait, DatabaseBackend, DbErr, FromQueryResult, Statement};

use crate::{
    repositories::database_connection::get_database_connection,
    sqls::billing::query_result::{CreateQueryResult, GetQueryResult},
};

const CREATE_BILLING_SP: &str = include_str!("./create_billing.sql");

pub async fn query(value: CreateQueryResult) -> Result<u64, DbErr> {
    let conn = get_database_connection().await;

    let stmt = Statement::from_sql_and_values(
        DatabaseBackend::Sqlite,
        CREATE_BILLING_SP,
        [
            value.customer_id.into(),
            value.date.into(),
            value.from.into(),
            value.to.into(),
        ],
    );
    let result = conn.execute_raw(stmt).await?;
    Ok(result.rows_affected())
}

