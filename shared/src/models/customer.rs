use chrono::NaiveDateTime;

pub struct Customer {
    pub customer_id: i64,
    pub customer_name: String,
    pub is_active: bool,
    pub is_admin: bool,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
}
