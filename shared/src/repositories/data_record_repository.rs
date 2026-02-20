use crate::{
    models::{
        data_record::{DataRecord, DefaultDataRecordType},
        to_active_without_id_trait::ToActiveModel,
    },
    repositories::{
        base_repository_trait::{BaseRepository, BaseRepositoryErr},
        database_connection::get_database_connection,
        generic_crud_repository::GenericCrudRepository,
    },
};
use ams_entity::data_record as data_record_db;
use ams_entity::prelude::DataRecord as DataRecordDb;
use chrono::{Datelike, Month, NaiveDate, NaiveDateTime};
use sea_orm::{ColumnTrait as _, EntityTrait as _, QueryFilter};

pub enum DataRecordRepositoryErr {
    FailToGetByDate,
    FailToGetByMonth,
}

pub struct DataRecordRepository {}

impl DataRecordRepository {
    pub async fn get_by_date(
        &mut self,
        date: NaiveDateTime,
    ) -> Result<Vec<DefaultDataRecordType>, DataRecordRepositoryErr> {
        let conn = get_database_connection().await;

        let date = NaiveDate::from_ymd_opt(date.year(), date.month(), date.day());

        let data = DataRecordDb::find()
            .filter(data_record_db::Column::Date.eq(date))
            .all(conn)
            .await
            .map_err(|_| DataRecordRepositoryErr::FailToGetByMonth)?;

        let data = data
            .iter()
            .map(|record| record.into())
            .collect::<Vec<DefaultDataRecordType>>();

        Ok(data)
    }

    pub async fn get_by_month(
        &mut self,
        year: i32,
        from: Month,
        to: Month,
    ) -> Result<Vec<DefaultDataRecordType>, DataRecordRepositoryErr> {
        let conn = get_database_connection().await;

        let from_month = NaiveDate::from_ymd_opt(year, from.number_from_month(), 1)
            .unwrap()
            .and_hms_opt(1, 0, 0)
            .unwrap();

        let to_month = from_month.with_month(to.number_from_month()).unwrap();

        let data = DataRecordDb::find()
            .filter(data_record_db::Column::Date.between(from_month, to_month))
            .all(conn)
            .await
            .map_err(|_| DataRecordRepositoryErr::FailToGetByMonth)?;

        let data = data
            .iter()
            .map(|record| record.into())
            .collect::<Vec<DefaultDataRecordType>>();

        Ok(data)
    }
}

impl BaseRepository<DefaultDataRecordType> for DataRecordRepository {
    async fn create(&mut self, model: DefaultDataRecordType) -> Result<i64, BaseRepositoryErr> {
        let active_model = model.to_active_without_id();
        let result = DataRecordDb.create(active_model).await;

        match result {
            Ok(created_model) => Ok(created_model.data_record_id),
            Err(_) => Err(BaseRepositoryErr::FailToCreate),
        }
    }

    async fn read(&mut self, id: i64) -> Result<Option<DefaultDataRecordType>, BaseRepositoryErr> {
        match DataRecordDb.get_by_id(id).await {
            Ok(model) => {
                let model = model.ok_or(BaseRepositoryErr::FailToRead)?;
                Ok(Some(model.into()))
            }
            Err(_) => Err(BaseRepositoryErr::FailToCreate),
        }
    }

    async fn update(
        &mut self,
        model: DefaultDataRecordType,
    ) -> Result<DefaultDataRecordType, BaseRepositoryErr> {
        let active_model = model.to_active_with_id();
        let update_result = DataRecordDb.update_by_model(active_model).await;

        match update_result {
            Ok(model) => Ok(model.into()),
            Err(_) => Err(BaseRepositoryErr::FailToUpdate),
        }
    }

    async fn delete(&mut self, id: i64) -> Result<u64, BaseRepositoryErr> {
        match DataRecordDb.delete_by_model_id(id).await {
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
