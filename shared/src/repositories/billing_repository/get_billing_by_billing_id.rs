use crate::{
    models::{
        balance::{BalanceWithCustomer, BalanceWithCustomerExtensionMethodTrait},
        billing::billing_info::BillingInfoWithBalance,
        customer::Customer,
        price::Price,
        retrieve_data::retrieve_data_with_customer_and_price::RetrieveDataWithCustomerAndPrice,
    },
    repositories::database_connection::get_database_connection,
};
use ams_entity::balance as balance_db;
use ams_entity::balance_billing as balance_billing_db;
use ams_entity::billing_retrieve_data as billing_retrieve_data_db;
use ams_entity::customer as customer_db;
use ams_entity::prelude::Balance as BalanceDb;
use ams_entity::prelude::Customer as CustomerDb;
use ams_entity::prelude::Price as PriceDb;
use ams_entity::prelude::RetrieveData as RetrieveDataDb;
use ams_entity::retrieve_data as retrieve_data_db;
use anyhow::Result;
use chrono::NaiveDateTime;
use sea_orm::{
    ColumnTrait, DatabaseBackend, DbErr, EntityTrait, FromQueryResult, JoinType, Order,
    QueryFilter, QueryOrder, QuerySelect, RelationTrait, Statement,
};

#[derive(Debug, Clone, FromQueryResult)]
pub struct GetQueryResult {
    pub billing_id: i64,
    pub balance_id: i64,
    pub customer_id: i64,

    pub bill: f32,
    pub amount: i64,

    pub paid_bill: f32,
    pub paid_total_amount: i64,

    pub unpaid_bill: f32,
    pub unpaid_total_amount: i64,

    pub from: NaiveDateTime,
    pub to: NaiveDateTime,
}

const GET_BILLING_BY_BILLING_ID: &str = include_str!("./get_billing_by_billing_id.sql");

pub async fn query(billing_id: i64) -> Result<BillingInfoWithBalance> {
    let conn = get_database_connection().await;

    let stmt = Statement::from_sql_and_values(
        DatabaseBackend::Sqlite,
        GET_BILLING_BY_BILLING_ID,
        [billing_id.into()],
    );

    let query_result = GetQueryResult::find_by_statement(stmt)
        .one(conn)
        .await?
        .ok_or(DbErr::Custom("fail to get by billing_id".into()))?;

    let customer: Customer = CustomerDb::find()
        .filter(customer_db::Column::CustomerId.eq(query_result.customer_id))
        .one(conn)
        .await?
        .ok_or(DbErr::Custom("fail to get customer".into()))?
        .into();

    let balance: BalanceWithCustomer = BalanceDb::find()
        .join(JoinType::Join, balance_db::Relation::BalanceBilling.def())
        .filter(balance_billing_db::Column::BillingId.eq(query_result.billing_id))
        .one(conn)
        .await?
        .ok_or(DbErr::Custom("fail to get balance".into()))?
        .with_customer(customer);

    let rd_cs_prs = RetrieveDataDb::find()
        .find_also_related(CustomerDb)
        .find_also_related(PriceDb)
        .order_by(retrieve_data_db::Column::Date, Order::Desc)
        .join(
            JoinType::Join,
            retrieve_data_db::Relation::BillingRetrieveData.def(),
        )
        .filter(billing_retrieve_data_db::Column::BillingId.eq(query_result.billing_id))
        .all(conn)
        .await?;

    let mut retrieve_data_wcp = Vec::<RetrieveDataWithCustomerAndPrice>::new();

    for rd_cs_pr in rd_cs_prs {
        let price: Price = rd_cs_pr
            .2
            .ok_or(DbErr::Custom(
                "cant find related price in retrieve data".into(),
            ))?
            .into();

        let customer: Customer = rd_cs_pr
            .1
            .ok_or(DbErr::Custom(
                "cant find related price in retrieve data".into(),
            ))?
            .into();
        let rd_w_cs_pr =
            RetrieveDataWithCustomerAndPrice::with_price_and_customer(rd_cs_pr.0, price, customer);

        retrieve_data_wcp.push(rd_w_cs_pr);
    }

    Ok(BillingInfoWithBalance {
        from: query_result.from,
        to: query_result.to,
        retrieve_data: retrieve_data_wcp,
        balance,
        paid_bill: query_result.paid_bill,
        paid_total_amount: query_result.paid_total_amount,
        unpaid_bill: query_result.unpaid_bill,
        unpaid_total_amount: query_result.unpaid_total_amount,
        bill: query_result.bill,
        amount: query_result.amount,
    })
}

#[cfg(test)]
mod test_get_billing_by_billing_id {
    use sea_orm::QueryTrait as _;

    use super::*;

    #[test]
    fn test_get_billing_by_billing_id_fn() {
        let retrieves_dat_query = RetrieveDataDb::find()
            .order_by(retrieve_data_db::Column::Date, Order::Desc)
            .join(
                JoinType::Join,
                retrieve_data_db::Relation::BillingRetrieveData.def(),
            )
            .filter(billing_retrieve_data_db::Column::BillingId.eq(1))
            .build(DatabaseBackend::Sqlite);
        println!("{}", retrieves_dat_query);
    }
}
