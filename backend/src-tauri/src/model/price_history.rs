use chrono::NaiveDate;

pub struct PriceHistory {
    pub id: u32,
    pub price: f64,
    pub changed_date: NaiveDate,
}
