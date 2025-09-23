use ams_entity::{prelude::*, taking_record_table};

use crate::repositories::abstract_repository_trait::AbstractRepository;

pub async fn add_new_taking_record(record: taking_record_table::Model) -> i32 {
    let active_model: taking_record_table::ActiveModel = record.into();

    let result = TakingRecordTable::create(active_model).await.unwrap();

    result.id
}
