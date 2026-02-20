use crate::models::{customer::Customer, to_active_without_id_trait::ToActiveModel};
use chrono::NaiveDateTime;
use sea_orm::ActiveValue::*;

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
