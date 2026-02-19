use crate::{
    models::customer::Customer,
    repositories::base_repository_trait::{BaseRepository, BaseRepositoryErr},
};

pub enum CustomerRepositoryErr {
    FailToGetAllActive,
}

pub struct CustomerRepository;

impl CustomerRepository {
    pub fn get_all_active(&mut self) -> Result<Vec<Customer>, CustomerRepositoryErr> {
        todo!()
    }
}

impl BaseRepository<Customer> for CustomerRepository {
    async fn create(&mut self, model: Customer) -> Result<i64, BaseRepositoryErr> {
        todo!()
    }

    async fn read(&mut self, id: i64) -> Result<Option<Customer>, BaseRepositoryErr> {
        todo!()
    }

    async fn update(&mut self, model: Customer) -> Result<Customer, BaseRepositoryErr> {
        todo!()
    }

    async fn delete(&mut self, id: i64) -> Result<u64, BaseRepositoryErr> {
        todo!()
    }
}
