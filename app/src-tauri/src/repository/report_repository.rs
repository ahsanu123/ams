use crate::model::product::Product;

use super::crud_repository_trait::CrudRepositoryTrait;

pub struct ReportRepository {}

impl CrudRepositoryTrait<Product> for ReportRepository {
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
    fn report_repository_get_all() {
        todo!();
    }

    #[test]
    fn report_repository_create() {
        todo!();
    }

    #[test]
    fn report_repository_read() {
        todo!();
    }

    #[test]
    fn report_repository_update() {
        todo!();
    }

    #[test]
    fn report_repository_delete() {
        todo!();
    }
}
