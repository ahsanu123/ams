use chrono::NaiveDate;

pub struct Production {
    pub id: u32,

    pub date: NaiveDate,
    pub product_price: f64,
    pub amount: u32,
    pub material_price: f64,
    pub description: String,
}
