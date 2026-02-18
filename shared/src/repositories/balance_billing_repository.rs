use crate::repositories::base_repository_trait::{BaseRepository, BaseRepositoryErr};
use ams_entity::prelude::BalanceBilling;

pub enum BalanceBillingRepositoryErr {
    FailToGetByCustomerId,
}

pub struct BalanceBillingRepository;

impl BalanceBillingRepository {
    pub fn get_by_customer_id(
        &mut self,
        customer_id: i64,
    ) -> Result<Vec<BalanceBilling>, BalanceBillingRepositoryErr> {
        todo!()
    }

    pub fn get_by_year(
        &mut self,
        year: i32,
    ) -> Result<Vec<BalanceBilling>, BalanceBillingRepositoryErr> {
        todo!()
    }
}

impl BaseRepository<BalanceBilling> for BalanceBillingRepository {
    async fn create(&mut self, model: BalanceBilling) -> Result<i64, BaseRepositoryErr> {
        todo!()
    }

    async fn read(&mut self, id: i64) -> Result<BalanceBilling, BaseRepositoryErr> {
        todo!()
    }

    async fn update(&mut self, model: BalanceBilling) -> Result<BalanceBilling, BaseRepositoryErr> {
        todo!()
    }

    async fn delete(&mut self, id: i64) -> Result<i64, BaseRepositoryErr> {
        todo!()
    }
}
