use chrono::Local;

use crate::{
    models::customer::Customer,
    repositories::{CUSTOMER_REPO, base_repository_trait::BaseRepository},
};

pub trait CustomerControllerTrait {
    async fn create_new_customer(&mut self, customer_name: String) -> i64;
    async fn get_all_customer(&mut self) -> Vec<Customer>;
    async fn get_all_active_customer(&mut self) -> Vec<Customer>;
    async fn upsert_customer(&mut self, customer: Customer) -> i64;
    async fn get_by_customer_id(&mut self, customer_id: i64) -> Option<Customer>;
}

pub struct CustomerController;

impl CustomerControllerTrait for CustomerController {
    async fn create_new_customer(&mut self, customer_name: String) -> i64 {
        let customer = Customer {
            customer_id: 0,
            customer_name,
            is_active: true,
            is_admin: false,
            created_date: Local::now().naive_local(),
            updated_date: Local::now().naive_local(),
        };

        CUSTOMER_REPO
            .lock()
            .await
            .create(customer)
            .await
            .unwrap_or_default()
    }

    async fn get_all_customer(&mut self) -> Vec<Customer> {
        CUSTOMER_REPO
            .lock()
            .await
            .get_all()
            .await
            .unwrap_or_default()
    }

    async fn get_all_active_customer(&mut self) -> Vec<Customer> {
        CUSTOMER_REPO
            .lock()
            .await
            .get_all_active()
            .await
            .unwrap_or_default()
    }

    async fn upsert_customer(&mut self, customer: Customer) -> i64 {
        CUSTOMER_REPO
            .lock()
            .await
            .update(customer)
            .await
            .unwrap_or_default()
            .customer_id
    }

    async fn get_by_customer_id(&mut self, customer_id: i64) -> Option<Customer> {
        CUSTOMER_REPO
            .lock()
            .await
            .read(customer_id)
            .await
            .unwrap_or_default()
    }
}
