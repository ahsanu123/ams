use chrono::NaiveDate;

pub struct TakingRecord {
    pub id: i32,
    pub username: String,
    pub from_date: NaiveDate,
    pub to_date: NaiveDate,
    pub amount: i32,
    pub total_price: f64,
}
