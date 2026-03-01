use crate::{
    models::{
        customer::Customer,
        retrieve_data::retrieve_data_with_customer_and_price::RetrieveDataWithCustomerAndPrice,
        to_active_model_trait::ToActiveModel,
    },
    sqls::billing::{self, query_result::GetQueryResult},
};
use chrono::NaiveDateTime;
use sea_orm::ActiveValue::*;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub mod billing_info;
pub mod billing_with_retrieve_data;

#[derive(Debug, Default, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct Billing {
    pub billing_id: i64,
    pub customer_id: i64,

    #[ts(type = "Date")]
    pub date: NaiveDateTime,

    pub customer: Customer,
    #[ts(type = "Date")]
    pub from: NaiveDateTime,
    #[ts(type = "Date")]
    pub to: NaiveDateTime,
    pub bill: f64,
    pub amount: i64,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct BillingWithRetrieveData {
    pub billing_id: i64,
    pub customer_id: i64,

    #[ts(type = "Date")]
    pub date: NaiveDateTime,

    pub customer: Customer,
    #[ts(type = "Date")]
    pub from: NaiveDateTime,
    #[ts(type = "Date")]
    pub to: NaiveDateTime,
    pub bill: f64,
    pub amount: i64,

    pub retrieves_data: Vec<RetrieveDataWithCustomerAndPrice>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct BillingCreate {
    pub customer_id: i64,
    #[ts(type = "Date")]
    pub date: NaiveDateTime,

    #[ts(type = "Date")]
    pub from: NaiveDateTime,
    #[ts(type = "Date")]
    pub to: NaiveDateTime,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct BillingUpdate {
    pub billing_id: i64,
    pub customer_id: i64,
    #[ts(type = "Date")]
    pub date: NaiveDateTime,

    #[ts(type = "Date")]
    pub from: NaiveDateTime,
    #[ts(type = "Date")]
    pub to: NaiveDateTime,
}

impl Billing {
    pub fn from_query_result(query_result: GetQueryResult, customer: Customer) -> Self {
        Self {
            billing_id: query_result.billing_id,
            customer_id: query_result.customer_id,
            date: query_result.date,

            customer,

            from: query_result.from,
            to: query_result.to,
            amount: query_result.amount,
            bill: query_result.bill,
        }
    }
}

impl From<Billing> for billing::query_result::CreateQueryResult {
    fn from(value: Billing) -> Self {
        Self {
            customer_id: value.customer_id,
            date: value.date,
            from: value.from,
            to: value.to,
        }
    }
}

impl From<BillingCreate> for billing::query_result::CreateQueryResult {
    fn from(value: BillingCreate) -> Self {
        Self {
            customer_id: value.customer_id,
            date: value.date,
            from: value.from,
            to: value.to,
        }
    }
}

impl ToActiveModel<ams_entity::billing::ActiveModel> for Billing {
    fn to_active_without_id(&self) -> ams_entity::billing::ActiveModel {
        ams_entity::billing::ActiveModel {
            billing_id: NotSet,
            customer_id: Set(self.customer_id),
            date: Set(self.date),
        }
    }

    fn to_active_with_id(&self) -> ams_entity::billing::ActiveModel {
        ams_entity::billing::ActiveModel {
            billing_id: Set(self.billing_id),
            customer_id: Set(self.customer_id),
            date: Set(self.date),
        }
    }
}
