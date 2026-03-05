use crate::{
    models::{
        billing::billing_info::BillingInfo,
        customer::Customer,
        price::Price,
        retrieve_data::{
            RetrieveData, retrieve_data_with_customer_and_price::RetrieveDataWithCustomerAndPrice,
        },
    },
    repositories::database_connection::get_database_connection,
    shared_fn::assign_to_parrent_arr::assign_to_parent_arr,
};
use ams_entity::prelude::RetrieveData as RetrieveDataDb;
use ams_entity::retrieve_data as retrieve_data_db;
use chrono::NaiveDateTime;
use sea_orm::{
    ColumnTrait, DatabaseBackend, DbErr, EntityTrait, FromQueryResult, Order, QueryFilter,
    QueryOrder, Statement,
};

#[derive(Debug, Clone, FromQueryResult)]
pub struct GetQueryResult {
    pub customer_id: i64,
    pub customer_name: String,
    pub is_active: bool,
    pub is_admin: bool,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,

    pub price_id: i64,
    pub price_date: NaiveDateTime,
    pub value: f32,

    pub paid_bill: f32,
    pub paid_total_amount: i64,

    pub unpaid_bill: f32,
    pub unpaid_total_amount: i64,

    pub bill: f32,
    pub total_amount: i64,

    pub from: NaiveDateTime,
    pub to: NaiveDateTime,
}

const GET_BILLING_INFO_BY_CUSTOMER_ID: &str = include_str!("./get_billing_info_by_customer_id.sql");

pub async fn query(customer_id: i64) -> Result<Vec<BillingInfo>, DbErr> {
    let conn = get_database_connection().await;

    let stmt = Statement::from_sql_and_values(
        DatabaseBackend::Sqlite,
        GET_BILLING_INFO_BY_CUSTOMER_ID,
        [customer_id.into()],
    );

    let query_results = GetQueryResult::find_by_statement(stmt).all(conn).await?;

    let cs_pr: Vec<(
        Customer,
        Price,
        f32,
        i64,
        NaiveDateTime,
        NaiveDateTime,
        f32,
        i64,
        f32,
        i64,
    )> = query_results
        .iter()
        .map(|qr| {
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

            (
                customer,
                price,
                qr.bill,
                qr.total_amount,
                qr.from,
                qr.to,
                qr.unpaid_bill,
                qr.unpaid_total_amount,
                qr.paid_bill,
                qr.paid_total_amount,
            )
        })
        .collect();

    let retrieves_data = RetrieveDataDb::find()
        .filter(retrieve_data_db::Column::CustomerId.eq(customer_id))
        .order_by(retrieve_data_db::Column::Date, Order::Desc)
        .all(conn)
        .await?
        .iter()
        .map(|model| model.into())
        .collect::<Vec<RetrieveData>>();

    let billing_infos = assign_to_parent_arr(
        retrieves_data,
        cs_pr,
        |data| data.customer_id,
        |p_data| p_data.0.customer_id,
        |query_result, retrieves_data| BillingInfo {
            from: query_result.4,
            to: query_result.5,
            retrieve_data: retrieves_data
                .iter()
                .map(|retrieve_data| {
                    retrieve_data
                        .clone()
                        .with_customer_and_price(query_result.0.clone(), query_result.1.clone())
                })
                .collect::<Vec<RetrieveDataWithCustomerAndPrice>>(),
            bill: query_result.2,
            amount: query_result.3,
            unpaid_bill: query_result.6,
            unpaid_total_amount: query_result.7,
            paid_bill: query_result.8,
            paid_total_amount: query_result.9,
        },
    );

    Ok(billing_infos)
}
