use crate::model::{
    product::{Product, ProductTable},
    summary_information::SummaryInformation,
    user::User,
};
use crate::repository::crud_repository_trait::CrudRepositoryTrait;
use chrono::NaiveDate;

pub trait ProductRepositoryTrait {
    async fn change_product_price(&self, new_price: f64);
    async fn add_product_for_user(&self, user: User, amount: u32);
    async fn set_paid_status(&self, user: User, from: NaiveDate, to: NaiveDate, status: bool);
    async fn get_list_ampas_for_user(&self, user: User, date: NaiveDate) -> Vec<Product>;
    async fn get_bill_for_user(&self, user: User, date: NaiveDate) -> f64;
    async fn get_product_price(&self) -> f64;
    async fn get_product_price_history(&self) -> Vec<f64>;
    async fn get_summary_for_user(
        &self,
        user: User,
        date: NaiveDate,
        from: NaiveDate,
        to: NaiveDate,
    ) -> Vec<SummaryInformation>;
}

pub struct ProductRepository {}

impl ProductRepositoryTrait for ProductRepository {
    async fn change_product_price(&self, new_price: f64) {
        todo!()
    }

    async fn add_product_for_user(&self, user: User, amount: u32) {
        todo!()
    }

    async fn set_paid_status(&self, user: User, from: NaiveDate, to: NaiveDate, status: bool) {
        todo!()
    }

    async fn get_list_ampas_for_user(&self, user: User, date: NaiveDate) -> Vec<Product> {
        todo!()
    }

    async fn get_bill_for_user(&self, user: User, date: NaiveDate) -> f64 {
        todo!()
    }

    async fn get_product_price(&self) -> f64 {
        todo!()
    }

    async fn get_product_price_history(&self) -> Vec<f64> {
        todo!()
    }

    async fn get_summary_for_user(
        &self,
        user: User,
        date: NaiveDate,
        from: NaiveDate,
        to: NaiveDate,
    ) -> Vec<SummaryInformation> {
        todo!()
    }
}

impl Default for ProductRepository {
    fn default() -> Self {
        Self {}
    }
}

impl CrudRepositoryTrait<Product> for ProductRepository {
    async fn create(&self, data: &Product) {}

    async fn read(&self, id: u32) {
        todo!()
    }

    async fn update(&self, data: &Product) {
        todo!()
    }

    async fn delete(&self, id: u32) {
        todo!()
    }
}
