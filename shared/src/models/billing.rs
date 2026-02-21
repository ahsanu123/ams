use crate::{
    models::{customer::Customer, to_active_without_id_trait::ToActiveModel},
    sqls::billing::get_by_customer_id_query_result,
};
use chrono::NaiveDateTime;
use sea_orm::ActiveValue::*;

#[derive(Debug)]
pub struct Billing {
    pub billing_id: i64,
    pub customer_id: i64,
    pub date: NaiveDateTime,

    pub customer: Customer,
    pub from: NaiveDateTime,
    pub to: NaiveDateTime,
    pub bill: f64,
    pub amount: i64,
}

impl Billing {
    pub fn from_query_result(
        query_result: get_by_customer_id_query_result::QueryResult,
        customer: Customer,
    ) -> Self {
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
