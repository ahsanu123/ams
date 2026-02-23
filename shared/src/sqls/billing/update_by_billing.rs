use sea_orm::{ConnectionTrait, DatabaseBackend, DbErr, Statement};

use crate::{
    models::billing::BillingUpdate, repositories::database_connection::get_database_connection,
};

const UPDATE_BY_BILLING_SP: &str = include_str!("./update_by_billing.sql");

pub async fn query(billing: BillingUpdate) -> Result<u64, DbErr> {
    let conn = get_database_connection().await;

    let stmt = Statement::from_sql_and_values(
        DatabaseBackend::Sqlite,
        UPDATE_BY_BILLING_SP,
        [
            billing.customer_id.into(),
            billing.date.into(),
            billing.billing_id.into(),
            billing.billing_id.into(),
            billing.billing_id.into(),
            billing.from.into(),
            billing.to.into(),
        ],
    );

    let result = conn.execute_raw(stmt).await?;

    Ok(result.rows_affected())
}
