use crate::{
    models::{retrieve_data::RetrieveData, to_active_without_id_trait::ToActiveModel},
    repositories::{
        base_repository_trait::{BaseRepository, BaseRepositoryErr},
        database_connection::get_database_connection,
        generic_crud_repository::{GenericCrudRepository as _, GenericCrudRepositoryWithRelation},
    },
};
use ams_entity::{customer, price};
use chrono::{Month, NaiveDateTime};
use sea_orm::{ModelTrait, Related};

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
        let active_model = model.to_active_without_id();

        let result = ams_entity::prelude::RetrieveData.create(active_model).await;

        match result {
            Ok(created_model) => Ok(created_model.retrieve_data_id),
            Err(_) => Err(BaseRepositoryErr::FailToCreate),
        }
    }

    async fn read(&mut self, id: i64) -> Result<Option<RetrieveData>, BaseRepositoryErr> {
        match ams_entity::prelude::RetrieveData.get_by_id(id).await {
            Ok(model) => {
                let mut model = model.ok_or(BaseRepositoryErr::FailToRead)?;

                let price = model
                    .find_related_one(price::Entity)
                    .await
                    .map_err(|_| BaseRepositoryErr::FailToGetRelated)?;

                let customer = model
                    .find_related_one(customer::Entity)
                    .await
                    .map_err(|_| BaseRepositoryErr::FailToGetRelated)?;

                let retrieve_data =
                    RetrieveData::with_price_and_customer(model, price.into(), customer.into());

                Ok(Some(retrieve_data))
            }
            Err(_) => Err(BaseRepositoryErr::FailToCreate),
        }
    }

    async fn update(&mut self, model: RetrieveData) -> Result<RetrieveData, BaseRepositoryErr> {
        let active_model = model.to_active_with_id();
        let update_result = ams_entity::prelude::RetrieveData
            .update_by_model(active_model)
            .await;

        match update_result {
            Ok(mut model) => {
                let price = model
                    .find_related_one(price::Entity)
                    .await
                    .map_err(|_| BaseRepositoryErr::FailToGetRelated)?;

                let customer = model
                    .find_related_one(customer::Entity)
                    .await
                    .map_err(|_| BaseRepositoryErr::FailToGetRelated)?;

                let retrieve_data =
                    RetrieveData::with_price_and_customer(model, price.into(), customer.into());

                Ok(retrieve_data)
            }
            Err(_) => Err(BaseRepositoryErr::FailToUpdate),
        }
    }

    async fn delete(&mut self, id: i64) -> Result<u64, BaseRepositoryErr> {
        match ams_entity::prelude::RetrieveData
            .delete_by_model_id(id)
            .await
        {
            Ok(deleted_count) => {
                if deleted_count > 0 {
                    return Ok(deleted_count);
                }

                Err(BaseRepositoryErr::FailToDelete)
            }
            Err(_) => Err(BaseRepositoryErr::FailToDelete),
        }
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
