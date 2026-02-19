use chrono::NaiveDateTime;

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
