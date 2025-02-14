use chrono::NaiveDate;

pub struct Product {
    pub id: u32,
    pub user_id: u32,
    pub paid: bool,
    pub production_date: NaiveDate,
    pub taken_date: NaiveDate,
    pub price: f64,
    pub amount: u32,
    pub description: String,
}
