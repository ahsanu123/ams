use chrono::{Month, NaiveDateTime};

use crate::{
    models::data_record::DefaultDataRecordType,
    repositories::base_repository_trait::{BaseRepository, BaseRepositoryErr},
};

pub enum DataRecordRepositoryErr {
    FailToGetByDate,
}

pub struct DataRecordRepository {}

impl DataRecordRepository {
    pub fn get_by_date(
        &mut self,
        date: NaiveDateTime,
    ) -> Result<Vec<DefaultDataRecordType>, DataRecordRepositoryErr> {
        todo!()
    }

    pub fn get_by_month(
        &mut self,
        year: i32,
        month: Month,
    ) -> Result<Vec<DefaultDataRecordType>, DataRecordRepositoryErr> {
        todo!()
    }
}

impl BaseRepository<DefaultDataRecordType> for DataRecordRepository {
    async fn create(&mut self, model: DefaultDataRecordType) -> Result<i64, BaseRepositoryErr> {
        todo!()
    }

    async fn read(&mut self, id: i64) -> Result<Option<DefaultDataRecordType>, BaseRepositoryErr> {
        todo!()
    }

    async fn update(
        &mut self,
        model: DefaultDataRecordType,
    ) -> Result<DefaultDataRecordType, BaseRepositoryErr> {
        todo!()
    }

    async fn delete(&mut self, id: i64) -> Result<u64, BaseRepositoryErr> {
        todo!()
    }
}

// use ams_entity::{prelude::*, production_record_table};
// use chrono::{Datelike, Months, NaiveDateTime};
// use sea_orm::{entity::*, query::*};
//
// use crate::repositories::database_connection::get_database_connection;
//
// pub trait AdditionalProductionRecordTableMethodTrait {
//     async fn get_production_record_by_month(
//         &mut self,
//         date: NaiveDateTime,
//     ) -> Vec<production_record_table::Model>;
// }
//
// pub struct ProductionRecordRepository {}
//
// impl AdditionalProductionRecordTableMethodTrait for ProductionRecordRepository {
//     async fn get_production_record_by_month(
//         &mut self,
//         date: NaiveDateTime,
//     ) -> Vec<production_record_table::Model> {
//         let conn = get_database_connection().await;
//
//         let start_month = date
//             .with_day(1)
//             .unwrap()
//             .date()
//             .and_hms_opt(0, 0, 0)
//             .unwrap();
//
//         let end_month = date
//             .checked_add_months(Months::new(1))
//             .unwrap()
//             .with_day(1)
//             .unwrap()
//             .date()
//             .and_hms_opt(0, 0, 0)
//             .unwrap();
//
//         let records = ProductionRecordTable::find()
//             .filter(production_record_table::Column::Date.gte(start_month))
//             .filter(production_record_table::Column::Date.lt(end_month))
//             .all(conn)
//             .await
//             .unwrap();
//
//         records
//     }
// }
