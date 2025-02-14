use chrono::NaiveDate;

use crate::model::{product::Product, summary_information::SummaryInformation, user::User};

pub trait ProductRepositoryTrait {
    pub async fn change_product_price(new_price: f64);
    pub async fn add_product_for_user(user: User, amount: u32);
    pub async fn set_paid_status(user: User, from: NaiveDate, to: NaiveDate, status: bool);
    pub async fn get_list_ampas_for_user(user: User, date: NaiveDate) -> Vec<Product>;
    pub async fn get_bill_for_user(user: User, date: NaiveDate) -> f64;
    pub async fn get_product_price() -> f64;
    pub async fn get_product_price_history() -> Vec<f64>;
    pub async fn get_summary_for_user(
        user: user,
        date: NaiveDate,
        from: NaiveDate,
        to: NaiveDate,
    ) -> Vec<SummaryInformation>;
}
