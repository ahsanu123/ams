use chrono::{Month, NaiveDateTime};

use crate::{
    models::retrieve_data::RetrieveData,
    repositories::base_repository_trait::{BaseRepository, BaseRepositoryErr},
};

pub enum RetrieveDataRepositoryErr {
    FailToGetByCustomerId,
}

pub struct RetrieveDataRepository;

impl RetrieveDataRepository {
    pub fn get_by_customer_id(
        &mut self,
        id: i64,
    ) -> Result<Vec<RetrieveData>, RetrieveDataRepositoryErr> {
        todo!()
    }

    pub fn get_by_month(
        &mut self,
        year: i32,
        from: Month,
        to: Month,
    ) -> Result<Vec<RetrieveData>, RetrieveDataRepositoryErr> {
        todo!()
    }

    pub fn get_by_year(
        &mut self,
        year: i32,
    ) -> Result<Vec<RetrieveData>, RetrieveDataRepositoryErr> {
        todo!()
    }

    pub fn get_by_customer_id_and_month(
        &mut self,
        customer_id: i64,
        year: i32,
        from: Month,
        to: Month,
    ) -> Result<Vec<RetrieveData>, RetrieveDataRepositoryErr> {
        todo!()
    }

    pub fn get_by_customer_id_and_year(
        &mut self,
        customer_id: i64,
        year: i32,
    ) -> Result<Vec<RetrieveData>, RetrieveDataRepositoryErr> {
        todo!()
    }
}

impl BaseRepository<RetrieveData> for RetrieveDataRepository {
    async fn create(&mut self, model: RetrieveData) -> Result<i64, BaseRepositoryErr> {
        todo!()
    }

    async fn read(&mut self, id: i64) -> Result<RetrieveData, BaseRepositoryErr> {
        todo!()
    }

    async fn update(&mut self, model: RetrieveData) -> Result<RetrieveData, BaseRepositoryErr> {
        todo!()
    }

    async fn delete(&mut self, id: i64) -> Result<i64, BaseRepositoryErr> {
        todo!()
    }
}

// use crate::repositories::{
//     abstract_repository_trait::AbstractRepository, database_connection::get_database_connection,
// };
// use ams_entity::{prelude::*, taking_record_table};
// use chrono::NaiveDateTime;
// use sea_orm::{entity::*, query::*};
//
// pub trait AdditionalTakingRecordTableMethodTrait {
//     async fn get_taking_record_by_user_id(
//         &mut self,
//         user_id: i32,
//     ) -> Vec<taking_record_table::Model>;
//
//     async fn get_taking_record_by_date(
//         &mut self,
//         date: NaiveDateTime,
//     ) -> Vec<taking_record_table::Model>;
//
//     async fn upsert_taking_record(&mut self, taking_record: taking_record_table::Model) -> i32;
//
//     async fn get_taking_record_by_user_id_and_date(
//         &mut self,
//         user_id: i32,
//         date: NaiveDateTime,
//     ) -> Vec<taking_record_table::Model>;
// }
//
// pub struct TakingRecordRepository {
//     taking_record_table: TakingRecordTable,
// }
//
// impl TakingRecordRepository {
//     pub fn new(taking_record_table: TakingRecordTable) -> Self {
//         Self {
//             taking_record_table,
//         }
//     }
// }
//
// impl AdditionalTakingRecordTableMethodTrait for TakingRecordRepository {
//     async fn get_taking_record_by_user_id(
//         &mut self,
//         user_id: i32,
//     ) -> Vec<taking_record_table::Model> {
//         let conn = get_database_connection().await;
//
//         let result = TakingRecordTable::find()
//             .filter(taking_record_table::Column::UserId.eq(user_id))
//             .all(conn)
//             .await
//             .unwrap();
//
//         result
//     }
//
//     async fn get_taking_record_by_date(
//         &mut self,
//         date: NaiveDateTime,
//     ) -> Vec<taking_record_table::Model> {
//         let conn = get_database_connection().await;
//
//         let records = TakingRecordTable::find()
//             .filter(taking_record_table::Column::TakenDate.eq(date))
//             .all(conn)
//             .await
//             .unwrap();
//
//         records
//     }
//
//     async fn upsert_taking_record(&mut self, taking_record: taking_record_table::Model) -> i32 {
//         let record_exists = self
//             .taking_record_table
//             .get_by_id(taking_record.id)
//             .await
//             .unwrap();
//
//         let active_model: taking_record_table::ActiveModel = taking_record.into();
//         if record_exists.is_some() {
//             let updated_result = self
//                 .taking_record_table
//                 .update_by_model(active_model)
//                 .await
//                 .unwrap();
//
//             return updated_result.id;
//         } else {
//             let result = self.taking_record_table.create(active_model).await.unwrap();
//             return result.id;
//         }
//     }
//
//     async fn get_taking_record_by_user_id_and_date(
//         &mut self,
//         user_id: i32,
//         date: NaiveDateTime,
//     ) -> Vec<taking_record_table::Model> {
//         let conn = get_database_connection().await;
//
//         let result = TakingRecordTable::find()
//             .filter(taking_record_table::Column::UserId.eq(user_id))
//             .filter(taking_record_table::Column::TakenDate.eq(date))
//             .all(conn)
//             .await
//             .unwrap();
//
//         result
//     }
// }
