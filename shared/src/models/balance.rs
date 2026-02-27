use crate::models::{customer::Customer, to_active_without_id_trait::ToActiveModel};
use chrono::NaiveDateTime;
use sea_orm::ActiveValue::{NotSet, Set};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Copy, Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
#[ts(type = "number")]
pub enum TransactionType {
    #[ts(type = "number")]
    TopUp,
    #[ts(type = "number")]
    Pay,
}

impl From<i64> for TransactionType {
    fn from(value: i64) -> Self {
        match value {
            0 => TransactionType::TopUp,
            1 => TransactionType::Pay,
            val => panic!("cant find coresponding TransactionType for {}", val),
        }
    }
}

impl From<TransactionType> for i64 {
    fn from(value: TransactionType) -> Self {
        match value {
            TransactionType::TopUp => 0,
            TransactionType::Pay => 1,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct Balance {
    pub balance_id: i64,
    pub customer_id: i64,
    pub value: i64,
    #[ts(type = "Date")]
    pub date: NaiveDateTime,
    pub transaction_type: TransactionType,
    pub customer: Customer,
}

impl Balance {
    pub fn with_customer(model: ams_entity::balance::Model, customer: Customer) -> Self {
        Self {
            balance_id: model.balance_id,
            customer_id: model.customer_id,
            value: model.value,
            date: model.date,
            transaction_type: model.transaction_type.into(),

            customer,
        }
    }
}

impl ToActiveModel<ams_entity::balance::ActiveModel> for Balance {
    fn to_active_without_id(&self) -> ams_entity::balance::ActiveModel {
        ams_entity::balance::ActiveModel {
            balance_id: NotSet,
            customer_id: Set(self.customer_id),
            value: Set(self.value),
            date: Set(self.date),
            transaction_type: Set(self.transaction_type.into()),
        }
    }

    fn to_active_with_id(&self) -> ams_entity::balance::ActiveModel {
        ams_entity::balance::ActiveModel {
            balance_id: Set(self.balance_id),
            customer_id: Set(self.customer_id),
            value: Set(self.value),
            date: Set(self.date),
            transaction_type: Set(self.transaction_type.into()),
        }
    }
}
