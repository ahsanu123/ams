use crate::model::product::Product;

use super::crud_repository_trait::CrudRepositoryTrait;
use ams_lib::model::paid_status::PaidStatus;
use chrono::NaiveDate;
use diesel::Connection;

pub struct PaymentRepository {}

impl PaymentRepository {
    fn bulkUpdate(&self, from: &NaiveDate, to: &NaiveDate) {
        todo!()
    }
}

impl CrudRepositoryTrait<Product> for PaymentRepository {
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
    fn payment_repository_get_all() {
        todo!();
    }

    #[test]
    fn payment_repository_create() {
        todo!();
    }

    #[test]
    fn payment_repository_read() {
        todo!();
    }

    #[test]
    fn payment_repository_update() {
        todo!();
    }

    #[test]
    fn payment_repository_delete() {
        todo!();
    }
}
