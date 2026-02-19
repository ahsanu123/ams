use chrono::NaiveDateTime;

pub struct Customer {
    pub customer_id: i64,
    pub customer_name: String,
    pub is_active: bool,
    pub is_admin: bool,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
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
