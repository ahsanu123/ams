use crate::models::{customer::Customer, to_active_model_trait::ToActiveModel};
use chrono::{Local, NaiveDateTime};
use sea_orm::ActiveValue::{NotSet, Set};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::ToSchema;

#[derive(Copy, PartialEq, Eq, Debug, Serialize, Deserialize, Clone, TS, ToSchema)]
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

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, TS, ToSchema)]
#[ts(export)]
pub struct BalanceCreateOrUpdateWithoutChangedValue {
    pub balance_id: i64,
    pub customer_id: i64,
    pub value: f32,
    #[ts(type = "Date")]
    pub date: NaiveDateTime,
    pub transaction_type: TransactionType,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, TS, ToSchema)]
#[ts(export)]
pub struct BalanceCreateOrUpdate {
    pub balance_id: i64,
    pub customer_id: i64,
    pub value: f32,
    pub changed_value: f32,
    #[ts(type = "Date")]
    pub date: NaiveDateTime,
    pub transaction_type: TransactionType,
}

impl BalanceCreateOrUpdate {
    pub fn empty_balance(customer_id: i64) -> Self {
        Self {
            balance_id: 0,
            customer_id,
            value: 0.0,
            changed_value: 0.0,
            date: Local::now().naive_local(),
            transaction_type: TransactionType::TopUp,
        }
    }
}

impl BalanceCreateOrUpdateWithoutChangedValue {
    pub fn with_changed_value(&mut self, changed_value: f32) -> BalanceCreateOrUpdate {
        BalanceCreateOrUpdate {
            balance_id: self.balance_id,
            customer_id: self.customer_id,
            value: self.value,
            changed_value,
            date: self.date,
            transaction_type: self.transaction_type,
        }
    }
}

impl ToActiveModel<ams_entity::balance::ActiveModel> for BalanceCreateOrUpdate {
    fn to_active_without_id(&self) -> ams_entity::balance::ActiveModel {
        ams_entity::balance::ActiveModel {
            balance_id: NotSet,
            customer_id: Set(self.customer_id),
            value: Set(self.value),
            date: Set(Local::now().naive_local()),
            transaction_type: Set(self.transaction_type.into()),
            changed_value: Set(self.changed_value),
        }
    }

    fn to_active_with_id(&self) -> ams_entity::balance::ActiveModel {
        ams_entity::balance::ActiveModel {
            balance_id: Set(self.balance_id),
            customer_id: Set(self.customer_id),
            value: Set(self.value),
            date: Set(self.date),
            transaction_type: Set(self.transaction_type.into()),
            changed_value: Set(self.changed_value),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct BalanceWithCustomer {
    pub balance_id: i64,
    pub customer_id: i64,
    pub value: f32,
    pub changed_value: f32,
    #[ts(type = "Date")]
    pub date: NaiveDateTime,
    pub transaction_type: TransactionType,
    pub customer: Customer,
}

impl BalanceWithCustomer {
    pub fn with_customer(model: ams_entity::balance::Model, customer: Customer) -> Self {
        Self {
            balance_id: model.balance_id,
            customer_id: model.customer_id,
            value: model.value,
            changed_value: model.changed_value,
            date: model.date,
            transaction_type: model.transaction_type.into(),

            customer,
        }
    }
}

pub trait BalanceWithCustomerExtensionMethodTrait {
    fn with_customer(&mut self, customer: Customer) -> BalanceWithCustomer;
}

impl BalanceWithCustomerExtensionMethodTrait for ams_entity::balance::Model {
    fn with_customer(&mut self, customer: Customer) -> BalanceWithCustomer {
        BalanceWithCustomer {
            balance_id: self.balance_id,
            customer_id: self.customer_id,
            value: self.value,
            changed_value: self.changed_value,
            date: self.date,
            transaction_type: self.transaction_type.into(),

            customer,
        }
    }
}

impl ToActiveModel<ams_entity::balance::ActiveModel> for BalanceWithCustomer {
    fn to_active_without_id(&self) -> ams_entity::balance::ActiveModel {
        ams_entity::balance::ActiveModel {
            balance_id: NotSet,
            customer_id: Set(self.customer_id),
            value: Set(self.value),
            date: Set(self.date),
            transaction_type: Set(self.transaction_type.into()),
            changed_value: Set(self.changed_value),
        }
    }

    fn to_active_with_id(&self) -> ams_entity::balance::ActiveModel {
        ams_entity::balance::ActiveModel {
            balance_id: Set(self.balance_id),
            customer_id: Set(self.customer_id),
            value: Set(self.value),
            date: Set(self.date),
            transaction_type: Set(self.transaction_type.into()),
            changed_value: Set(self.changed_value),
        }
    }
}
