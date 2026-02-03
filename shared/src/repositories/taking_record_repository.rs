use crate::repositories::{
    abstract_repository_trait::AbstractRepository, database_connection::get_database_connection,
};
use ams_entity::{prelude::*, taking_record_table};
use chrono::NaiveDateTime;
use sea_orm::{entity::*, query::*};

pub trait AdditionalTakingRecordTableMethodTrait {
    async fn get_taking_record_by_user_id(
        &mut self,
        user_id: i32,
    ) -> Vec<taking_record_table::Model>;

    async fn get_taking_record_by_date(
        &mut self,
        date: NaiveDateTime,
    ) -> Vec<taking_record_table::Model>;

    async fn upsert_taking_record(&mut self, taking_record: taking_record_table::Model) -> i32;

    async fn get_taking_record_by_user_id_and_date(
        &mut self,
        user_id: i32,
        date: NaiveDateTime,
    ) -> Vec<taking_record_table::Model>;
}

pub struct TakingRecordRepository {
    taking_record_table: TakingRecordTable,
}

impl TakingRecordRepository {
    pub fn new(taking_record_table: TakingRecordTable) -> Self {
        Self {
            taking_record_table,
        }
    }
}

impl AdditionalTakingRecordTableMethodTrait for TakingRecordRepository {
    async fn get_taking_record_by_user_id(
        &mut self,
        user_id: i32,
    ) -> Vec<taking_record_table::Model> {
        let conn = get_database_connection().await;

        let result = TakingRecordTable::find()
            .filter(taking_record_table::Column::UserId.eq(user_id))
            .all(conn)
            .await
            .unwrap();

        result
    }

    async fn get_taking_record_by_date(
        &mut self,
        date: NaiveDateTime,
    ) -> Vec<taking_record_table::Model> {
        let conn = get_database_connection().await;

        let records = TakingRecordTable::find()
            .filter(taking_record_table::Column::TakenDate.eq(date))
            .all(conn)
            .await
            .unwrap();

        records
    }

    async fn upsert_taking_record(&mut self, taking_record: taking_record_table::Model) -> i32 {
        let record_exists = self
            .taking_record_table
            .get_by_id(taking_record.id)
            .await
            .unwrap();

        let active_model: taking_record_table::ActiveModel = taking_record.into();
        if record_exists.is_some() {
            let updated_result = self
                .taking_record_table
                .update_by_model(active_model)
                .await
                .unwrap();

            return updated_result.id;
        } else {
            let result = self.taking_record_table.create(active_model).await.unwrap();
            return result.id;
        }
    }

    async fn get_taking_record_by_user_id_and_date(
        &mut self,
        user_id: i32,
        date: NaiveDateTime,
    ) -> Vec<taking_record_table::Model> {
        let conn = get_database_connection().await;

        let result = TakingRecordTable::find()
            .filter(taking_record_table::Column::UserId.eq(user_id))
            .filter(taking_record_table::Column::TakenDate.eq(date))
            .all(conn)
            .await
            .unwrap();

        result
    }
}
