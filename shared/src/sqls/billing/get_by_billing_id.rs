use sea_orm::{DatabaseBackend, DbErr, FromQueryResult, Statement};

use crate::{
    repositories::database_connection::get_database_connection,
    sqls::billing::query_result::GetQueryResult,
};

const GET_BY_BILLING_ID_SP: &str = include_str!("./get_by_billing_id.sql");

pub async fn query(billing_id: i64) -> Result<Option<GetQueryResult>, DbErr> {
    let conn = get_database_connection().await;

    let stmt = Statement::from_sql_and_values(
        DatabaseBackend::Sqlite,
        GET_BY_BILLING_ID_SP,
        [billing_id.into()],
    );

    GetQueryResult::find_by_statement(stmt).one(conn).await
}
