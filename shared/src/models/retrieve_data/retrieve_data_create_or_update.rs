use std::i64;

use crate::models::{price::Price, to_active_model_trait::ToActiveModel};
use chrono::{Local, NaiveDateTime};
use sea_orm::ActiveValue::{NotSet, Set};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct RetrieveDataCreateOrUpdate {
    pub retrieve_data_id: i64,
    pub customer_id: i64,
    pub price_id: i64,
    pub amount: i64,
    #[ts(type = "Date")]
    pub date: NaiveDateTime,
    pub is_paid: bool,
}

impl RetrieveDataCreateOrUpdate {
    pub fn create_ca(customer_id: i64, amount: i64, price: Price) -> Self {
        Self {
            retrieve_data_id: 0,
            customer_id,
            price_id: price.price_id,
            amount,
            date: Local::now().naive_local(),
            is_paid: false,
        }
    }

    pub fn create_cawd(customer_id: i64, amount: i64, price: Price, date: NaiveDateTime) -> Self {
        Self {
            retrieve_data_id: 0,
            customer_id,
            price_id: price.price_id,
            amount,
            date,
            is_paid: false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, TS, ToSchema)]
#[ts(export)]
pub struct RetrieveDataCreate {
    pub customer_id: i64,
    pub amount: i64,
}

#[derive(Serialize, Deserialize, Clone, TS, ToSchema)]
#[ts(export)]
pub struct RetrieveDataCreateWithDate {
    pub customer_id: i64,
    pub amount: i64,
    #[ts(type = "Date")]
    pub date: NaiveDateTime,
}

impl ToActiveModel<ams_entity::retrieve_data::ActiveModel> for RetrieveDataCreateOrUpdate {
    fn to_active_without_id(&self) -> ams_entity::retrieve_data::ActiveModel {
        ams_entity::retrieve_data::ActiveModel {
            retrieve_data_id: NotSet,
            customer_id: Set(self.customer_id),
            price_id: Set(self.price_id),
            amount: Set(self.amount),
            date: Set(self.date),
            is_paid: Set(self.is_paid),
        }
    }

    fn to_active_with_id(&self) -> ams_entity::retrieve_data::ActiveModel {
        ams_entity::retrieve_data::ActiveModel {
            retrieve_data_id: Set(self.retrieve_data_id),
            customer_id: Set(self.customer_id),
            price_id: Set(self.price_id),
            amount: Set(self.amount),
            date: Set(self.date),
            is_paid: Set(self.is_paid),
        }
    }
}
