use crate::models::{customer::Customer, price::Price, to_active_without_id_trait::ToActiveModel};
use chrono::NaiveDateTime;
use sea_orm::ActiveValue::{NotSet, Set};

pub struct RetrieveData {
    pub retrieve_data_id: i64,
    pub customer_id: i64,
    pub price_id: i64,
    pub amount: i64,
    pub date: NaiveDateTime,
    pub is_paid: bool,
    pub customer: Customer,
    pub price: Price,
}

impl RetrieveData {
    pub fn with_price_and_customer(
        model: ams_entity::retrieve_data::Model,
        price: Price,
        customer: Customer,
    ) -> Self {
        Self {
            price,
            customer,
            retrieve_data_id: model.retrieve_data_id,
            customer_id: model.customer_id,
            price_id: model.price_id,
            amount: model.amount,
            date: model.date,
            is_paid: model.is_paid,
        }
    }
}

impl ToActiveModel<ams_entity::retrieve_data::ActiveModel> for RetrieveData {
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
