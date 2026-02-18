use crate::{
    models::billing_retrieve_data::BillingRetrieveData,
    repositories::base_repository_trait::{BaseRepository, BaseRepositoryErr},
};

pub enum BillingRetrieveDataRepositoryErr {
    FailToGetByBillingId,
}

pub struct BillingRetrieveDataRepository;

impl BillingRetrieveDataRepository {
    pub fn get_by_billing_id(
        &mut self,
        billing_id: i64,
    ) -> Result<Vec<BillingRetrieveData>, BillingRetrieveDataRepositoryErr> {
        todo!()
    }
}

impl BaseRepository<BillingRetrieveData> for BillingRetrieveDataRepository {
    async fn create(&mut self, model: BillingRetrieveData) -> Result<i64, BaseRepositoryErr> {
        todo!()
    }

    async fn read(&mut self, id: i64) -> Result<BillingRetrieveData, BaseRepositoryErr> {
        todo!()
    }

    async fn update(
        &mut self,
        model: BillingRetrieveData,
    ) -> Result<BillingRetrieveData, BaseRepositoryErr> {
        todo!()
    }

    async fn delete(&mut self, id: i64) -> Result<i64, BaseRepositoryErr> {
        todo!()
    }
}
