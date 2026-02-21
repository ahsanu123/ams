use chrono::NaiveDateTime;
use sea_orm::ActiveValue::{NotSet, Set};

use crate::models::to_active_without_id_trait::ToActiveModel;

#[derive(Clone, Debug)]
pub struct Customer {
    pub customer_id: i64,
    pub customer_name: String,
    pub is_active: bool,
    pub is_admin: bool,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
}

impl From<&ams_entity::customer::Model> for Customer {
    fn from(value: &ams_entity::customer::Model) -> Self {
        Self {
            customer_id: value.customer_id,
            customer_name: value.customer_name.clone(),
            is_active: value.is_active,
            is_admin: value.is_admin,
            created_date: value.created_date,
            updated_date: value.updated_date,
        }
    }
}

impl From<ams_entity::customer::Model> for Customer {
    fn from(value: ams_entity::customer::Model) -> Self {
        Self {
            customer_id: value.customer_id,
            customer_name: value.customer_name,
            is_active: value.is_active,
            is_admin: value.is_admin,
            created_date: value.created_date,
            updated_date: value.updated_date,
        }
    }
}

impl ToActiveModel<ams_entity::customer::ActiveModel> for Customer {
    fn to_active_without_id(&self) -> ams_entity::customer::ActiveModel {
        ams_entity::customer::ActiveModel {
            customer_id: NotSet,
            customer_name: Set(self.customer_name.clone()),
            is_active: Set(self.is_active),
            is_admin: Set(self.is_admin),
            created_date: Set(self.created_date),
            updated_date: Set(self.updated_date),
        }
    }

    fn to_active_with_id(&self) -> ams_entity::customer::ActiveModel {
        ams_entity::customer::ActiveModel {
            customer_id: Set(self.customer_id),
            customer_name: Set(self.customer_name.clone()),
            is_active: Set(self.is_active),
            is_admin: Set(self.is_admin),
            created_date: Set(self.created_date),
            updated_date: Set(self.updated_date),
        }
    }
}
