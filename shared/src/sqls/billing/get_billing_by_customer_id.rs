use std::{collections::HashMap, hash::Hash};

use crate::{
    models::{
        balance::{BalanceWithCustomer, BalanceWithCustomerExtensionMethodTrait},
        billing::billing_info::{BillingInfo, BillingInfoWithBalance},
        customer::Customer,
        price::Price,
        retrieve_data::{
            RetrieveData, retrieve_data_with_customer_and_price::RetrieveDataWithCustomerAndPrice,
        },
    },
    repositories::database_connection::get_database_connection,
    shared_fn::assign_to_parrent_arr::assign_to_parent_arr,
};
use ams_entity::balance_billing as balance_billing_db;
use ams_entity::billing_retrieve_data as billing_retrieve_data_db;
use ams_entity::customer as customer_db;
use ams_entity::prelude::Balance as BalanceDb;
use ams_entity::prelude::BalanceBilling as BalanceBillingDb;
use ams_entity::prelude::BillingRetrieveData as BillingRetrieveDataDb;
use ams_entity::prelude::Customer as CustomerDb;
use ams_entity::prelude::Price as PriceDb;
use ams_entity::prelude::RetrieveData as RetrieveDataDb;
use ams_entity::retrieve_data as retrieve_data_db;
use ams_entity::{balance as balance_db, price};
use chrono::NaiveDateTime;
use itertools::Itertools;
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

const GET_BILLING_BY_CUSTOMER_ID: &str = include_str!("./get_billing_by_customer_id.sql");

pub async fn query(customer_id: i64) -> Result<Vec<BillingInfoWithBalance>, DbErr> {
    let conn = get_database_connection().await;

    let stmt = Statement::from_sql_and_values(
        DatabaseBackend::Sqlite,
        GET_BILLING_BY_CUSTOMER_ID,
        [customer_id.into()],
    );

    let query_results = GetQueryResult::find_by_statement(stmt).all(conn).await?;

    let billing_ids: Vec<i64> = query_results
        .iter()
        .map(|result| result.billing_id)
        .collect();

    let grouped_brd_models: HashMap<
        i64,
        Vec<(
            billing_retrieve_data_db::Model,
            Vec<retrieve_data_db::Model>,
        )>,
    > = BillingRetrieveDataDb::find()
        .find_with_related(RetrieveDataDb)
        .filter(billing_retrieve_data_db::Column::BillingId.is_in(billing_ids.clone()))
        .all(conn)
        .await?
        .into_iter()
        .into_grouping_map_by(|brd| brd.0.billing_id)
        .collect();

    let grouped_bb_models: HashMap<
        i64,
        Vec<(balance_billing_db::Model, Option<balance_db::Model>)>,
    > = BalanceBillingDb::find()
        .find_also_related(BalanceDb)
        .filter(balance_billing_db::Column::BillingId.is_in(billing_ids.clone()))
        .order_by_desc(balance_db::Column::Date)
        .all(conn)
        .await?
        .into_iter()
        .into_grouping_map_by(|bb| bb.0.billing_id)
        .collect();

    let customer_ids: Vec<i64> = query_results
        .iter()
        .map(|result| result.customer_id)
        .collect();

    let customers: Vec<Customer> = CustomerDb::find()
        .filter(customer_db::Column::CustomerId.is_in(customer_ids))
        .all(conn)
        .await?
        .iter()
        .map(|model| model.into())
        .collect();

    let prices: Vec<Price> = PriceDb::find()
        .join(
            JoinType::Join,
            billing_retrieve_data_db::Relation::RetrieveData.def(),
        )
        .join(
            JoinType::Join,
            retrieve_data_db::Relation::BillingRetrieveData.def(),
        )
        .filter(billing_retrieve_data_db::Column::BillingId.is_in(billing_ids))
        .all(conn)
        .await?
        .iter()
        .map(|model| model.into())
        .collect();

    let mut billing_infos = Vec::<BillingInfoWithBalance>::new();

    for qr in query_results {
        if let Some(rd_models) = grouped_brd_models.get(&qr.billing_id)
            && let Some(bb_models) = grouped_bb_models.get(&qr.billing_id)
        {
            let rd_wcp_models = rd_models
                .iter()
                .flat_map(|(_, retrieve_data_model)| retrieve_data_model)
                .cloned()
                .collect::<Vec<retrieve_data_db::Model>>();

            let mut rd_wcps = Vec::<RetrieveDataWithCustomerAndPrice>::new();

            for rd_wcp_m in rd_wcp_models {
                let price = prices
                    .iter()
                    .find(|p| p.price_id == rd_wcp_m.price_id)
                    .ok_or(DbErr::Custom(
                        "fail to get price from retrieve data model".into(),
                    ))?
                    .clone();

                let customer = customers
                    .iter()
                    .find(|c| c.customer_id == rd_wcp_m.customer_id)
                    .ok_or(DbErr::Custom(
                        "fail to get customer from retrieve data model".into(),
                    ))?
                    .clone();

                rd_wcps.push(RetrieveDataWithCustomerAndPrice {
                    retrieve_data_id: rd_wcp_m.retrieve_data_id,
                    customer_id: rd_wcp_m.customer_id,
                    price_id: rd_wcp_m.price_id,
                    amount: rd_wcp_m.amount,
                    date: rd_wcp_m.date,
                    is_paid: rd_wcp_m.is_paid,
                    customer,
                    price,
                });
            }

            let balance = bb_models
                .first()
                .ok_or(DbErr::Custom("fail to get balance billing".into()))?
                .1
                .clone()
                .ok_or(DbErr::Custom("fail to get balance billing".into()))?;

            let customer = customers
                .iter()
                .find(|c| c.customer_id == qr.customer_id)
                .ok_or(DbErr::Custom("fail to get customer for balance".into()))?
                .clone();

            let balance = BalanceWithCustomer::with_customer(balance, customer);

            billing_infos.push(BillingInfoWithBalance {
                from: qr.from,
                to: qr.to,
                paid_bill: qr.paid_bill,
                paid_total_amount: qr.paid_total_amount,
                unpaid_bill: qr.unpaid_bill,
                unpaid_total_amount: qr.unpaid_total_amount,
                bill: qr.bill,
                amount: qr.amount,

                retrieve_data: rd_wcps,
                balance,
            });
        }
    }

    Ok(billing_infos)
}
