use super::crud_repository_trait::CrudRepositoryTrait;
use ams_lib::model::money_record::MoneyRecord;

pub struct MoneyRecordRepository {}

impl CrudRepositoryTrait<MoneyRecord> for MoneyRecordRepository {
    fn getAll(&self) -> Result<Vec<MoneyRecord>, String> {
        todo!()
    }

    fn create(&self, data: &MoneyRecord) -> Result<usize, String> {
        todo!()
    }

    fn read(&self, id: u32) -> Result<MoneyRecord, String> {
        todo!()
    }

    fn update(&self, data: &MoneyRecord) -> Result<usize, String> {
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
