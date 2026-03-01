use chrono::NaiveDateTime;
use sea_orm::{DatabaseBackend, DbErr, FromQueryResult, Statement};

use crate::{
    models::{customer::Customer, price::Price, retrieve_data::RetrieveData},
    repositories::database_connection::get_database_connection,
};

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

    pub price_date: NaiveDateTime,
    pub value: f32,

    pub bill: f64,
    pub total_amount: i64,

    pub from: NaiveDateTime,
    pub to: NaiveDateTime,
}

const GET_BILLING_INFO_BY_DATE: &str = include_str!("./get_billing_info_by_date.sql");

pub async fn query(
    start_date: NaiveDateTime,
    end_date: NaiveDateTime,
) -> Result<Vec<(RetrieveData, Customer, Price)>, DbErr> {
    let conn = get_database_connection().await;

    let stmt = Statement::from_sql_and_values(
        DatabaseBackend::Sqlite,
        GET_BILLING_INFO_BY_DATE,
        [start_date.into(), end_date.into()],
    );

    let query_results = GetQueryResult::find_by_statement(stmt).all(conn).await?;

    let rd_cs_pr: Vec<(RetrieveData, Customer, Price)> = query_results
        .iter()
        .map(|qr| {
            let retrieve_data = RetrieveData {
                retrieve_data_id: qr.retrieve_data_id,
                customer_id: qr.customer_id,
                price_id: qr.price_id,
                amount: qr.amount,
                date: qr.date,
                is_paid: qr.is_paid,
            };
            let customer = Customer {
                customer_id: qr.customer_id,
                customer_name: qr.customer_name.clone(),
                is_active: qr.is_active,
                is_admin: qr.is_admin,
                created_date: qr.created_date,
                updated_date: qr.updated_date,
            };
            let price = Price {
                price_id: qr.price_id,
                date: qr.price_date,
                value: qr.value,
            };

            (retrieve_data, customer, price)
        })
        .collect();

    Ok(rd_cs_pr)
}
