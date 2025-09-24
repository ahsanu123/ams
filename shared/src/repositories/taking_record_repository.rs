use ams_entity::{prelude::*, taking_record_table};
use chrono::NaiveDateTime;
use sea_orm::{entity::*, prelude::async_trait::async_trait, query::*};

use crate::repositories::{
    abstract_repository_trait::AbstractRepository, get_sql_connection_trait::GetSqlConnectionTrait,
};

#[async_trait]
pub trait AdditionalTakingRecordTableMethodTrait {
    async fn get_taking_record_by_user_id(user_id: i32) -> Vec<taking_record_table::Model>;
    async fn get_taking_record_by_date(date: NaiveDateTime) -> Vec<taking_record_table::Model>;
    async fn upsert_taking_record(taking_record: taking_record_table::Model) -> i32;
    async fn get_taking_record_by_user_id_and_date(
        user_id: i32,
        date: NaiveDateTime,
    ) -> Vec<taking_record_table::Model>;
}

#[async_trait]
impl AdditionalTakingRecordTableMethodTrait for TakingRecordTable {
    async fn get_taking_record_by_user_id(user_id: i32) -> Vec<taking_record_table::Model> {
        let conn = TakingRecordTable::get_connection().await;

        let result = TakingRecordTable::find()
            .filter(taking_record_table::Column::UserId.eq(user_id))
            .all(conn)
            .await
            .unwrap();

        result
    }

    async fn get_taking_record_by_date(date: NaiveDateTime) -> Vec<taking_record_table::Model> {
        let conn = TakingRecordTable::get_connection().await;

        let records = TakingRecordTable::find()
            .filter(taking_record_table::Column::TakenDate.eq(date))
            .all(conn)
            .await
            .unwrap();

        records
    }

    async fn upsert_taking_record(taking_record: taking_record_table::Model) -> i32 {
        let record_exists = TakingRecordTable::get_by_id(taking_record.id)
            .await
            .unwrap();

        let active_model: taking_record_table::ActiveModel = taking_record.into();
        if record_exists.is_some() {
            let updated_result = TakingRecordTable::update_by_model(active_model)
                .await
                .unwrap();

            return updated_result.id;
        } else {
            let result = TakingRecordTable::create(active_model).await.unwrap();
            return result.id;
        }
    }

    async fn get_taking_record_by_user_id_and_date(
        user_id: i32,
        date: NaiveDateTime,
    ) -> Vec<taking_record_table::Model> {
        let conn = TakingRecordTable::get_connection().await;

        let result = TakingRecordTable::find()
            .filter(taking_record_table::Column::UserId.eq(user_id))
            .filter(taking_record_table::Column::TakenDate.eq(date))
            .all(conn)
            .await
            .unwrap();

        result
    }
}
