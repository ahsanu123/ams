use chrono::NaiveDateTime;

pub struct Price {
    pub price_id: i64,
    pub date: NaiveDateTime,
    pub value: f32,
}
