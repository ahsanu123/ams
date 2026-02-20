use chrono::NaiveDateTime;
use sea_orm::ActiveValue::{NotSet, Set};

use crate::models::to_active_without_id_trait::ToActiveModel;

pub struct Price {
    pub price_id: i64,
    pub date: NaiveDateTime,
    pub value: f32,
}

impl From<ams_entity::price::Model> for Price {
    fn from(value: ams_entity::price::Model) -> Self {
        Self {
            price_id: value.price_id,
            date: value.date,
            value: value.value,
        }
    }
}
impl ToActiveModel<ams_entity::price::ActiveModel> for Price {
    fn to_active_without_id(&self) -> ams_entity::price::ActiveModel {
        ams_entity::price::ActiveModel {
            price_id: NotSet,
            date: Set(self.date),
            value: Set(self.value),
        }
    }

    fn to_active_with_id(&self) -> ams_entity::price::ActiveModel {
        ams_entity::price::ActiveModel {
            price_id: Set(self.price_id),
            date: Set(self.date),
            value: Set(self.value),
        }
    }
}
