use chrono::NaiveDate;

use crate::model::product::Product;

use super::crud_repository_trait::CrudRepositoryTrait;

pub trait ProductionRepositoryTrait {
    async fn add_production(amount: u32, date: NaiveDate);
    async fn get_production_error(date: NaiveDate);
}

pub struct ProductionRepository {}

impl CrudRepositoryTrait<Product> for ProductionRepository {
    fn getAll(&self) -> Result<Vec<Product>, String> {
        todo!()
    }

    fn create(&self, data: &Product) -> Result<usize, String> {
        todo!()
    }

    fn read(&self, id: u32) -> Result<Product, String> {
        todo!()
    }

    fn update(&self, data: &Product) -> Result<usize, String> {
        todo!()
    }

    fn delete(&self, id: u32) -> Result<usize, String> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn production_repository_get_all() {
        todo!();
    }

    #[test]
    fn production_repository_create() {
        todo!();
    }

    #[test]
    fn production_repository_read() {
        todo!();
    }

    #[test]
    fn production_repository_update() {
        todo!();
    }

    #[test]
    fn production_repository_delete() {
        todo!();
    }
}
