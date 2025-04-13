use super::crud_repository_trait::CrudRepositoryTrait;
use crate::model::dreg_price_history::DregPrice;

pub struct DregPriceRepository {}

impl CrudRepositoryTrait<DregPrice> for DregPriceRepository {
    fn getAll(&self) -> Result<Vec<DregPrice>, String> {
        todo!()
    }

    fn create(&self, data: &DregPrice) -> Result<usize, String> {
        todo!()
    }

    fn read(&self, id: u32) -> Result<DregPrice, String> {
        todo!()
    }

    fn update(&self, data: &DregPrice) -> Result<usize, String> {
        todo!()
    }

    fn delete(&self, id: u32) -> Result<usize, String> {
        todo!()
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn todo() {
        todo!();
    }
}
