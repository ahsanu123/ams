use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::models::data_record::DefaultDataRecordType;

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct DataRecordGetAllProps {}

pub trait DataRecordControllerTrait {
    async fn get_all(&mut self, props: DataRecordGetAllProps) -> Vec<DefaultDataRecordType>;
    async fn create(&mut self, data: DefaultDataRecordType) -> i64;
}

pub struct DataRecordController;

impl DataRecordControllerTrait for DataRecordController {
    async fn get_all(&mut self, props: DataRecordGetAllProps) -> Vec<DefaultDataRecordType> {
        todo!()
    }

    async fn create(&mut self, data: DefaultDataRecordType) -> i64 {
        todo!()
    }
}
