use crate::{
    models::{
        retrieve_data::{
            retrieve_data_create_or_update::RetrieveDataCreateOrUpdate,
            retrieve_data_with_customer_and_price::RetrieveDataWithCustomerAndPrice,
        },
        to_active_model_trait::ToActiveModel,
    },
    repositories::{
        base_repository_trait::{BaseRepository, BaseRepositoryErr},
        database_connection::get_database_connection,
        generic_crud_repository::{GenericCrudRepository as _, GenericCrudRepositoryWithRelation},
    },
};
use ams_entity::customer::Model as CustomerModel;
use ams_entity::prelude::RetrieveData as RetrieveDataDb;
use ams_entity::price::Model as PriceModel;
use ams_entity::retrieve_data as retrieve_data_db;
use ams_entity::retrieve_data::Model as RetrieveDataModel;
use ams_entity::{customer, price};
use chrono::{Datelike, Month, NaiveDate};
use sea_orm::{prelude::*, query::*};

pub enum RetrieveDataRepositoryErr {
    FailToGetByCustomerId,
    FailToGetByCustomerYear,
    FailToGetByCustomerIdAndYear,
    FailToGetByCustomerIdAndMonth,
    FailToGetByMonth,
    FailToGetByYear,
    FailToGetRelated(String),
}

pub struct RetrieveDataRepository;

impl RetrieveDataRepository {
    async fn map_retrieve_data_db_to_final_model(
        &mut self,
        models: Vec<(RetrieveDataModel, Option<PriceModel>, Option<CustomerModel>)>,
    ) -> Result<Vec<RetrieveDataWithCustomerAndPrice>, RetrieveDataRepositoryErr> {
        let mut retrieves_data = Vec::<RetrieveDataWithCustomerAndPrice>::new();

        for (model, price, customer) in models {
            let price = price.ok_or(RetrieveDataRepositoryErr::FailToGetRelated(
                "cant find price".into(),
            ))?;

            let customer = customer.ok_or(RetrieveDataRepositoryErr::FailToGetRelated(
                "cant find customer".into(),
            ))?;

            let retrieve_data = RetrieveDataWithCustomerAndPrice::with_price_and_customer(
                model,
                price.into(),
                customer.into(),
            );

            retrieves_data.push(retrieve_data);
        }

        Ok(retrieves_data)
    }

    pub async fn get_by_customer_id(
        &mut self,
        customer_id: i64,
    ) -> Result<Vec<RetrieveDataWithCustomerAndPrice>, RetrieveDataRepositoryErr> {
        let conn = get_database_connection().await;

        let data = RetrieveDataDb::find()
            .filter(retrieve_data_db::Column::CustomerId.eq(customer_id))
            .find_also_related(price::Entity)
            .find_also_related(customer::Entity)
            .all(conn)
            .await
            .map_err(|_| RetrieveDataRepositoryErr::FailToGetByCustomerId)?;

        let retrieves_data = self.map_retrieve_data_db_to_final_model(data).await?;

        Ok(retrieves_data)
    }

    pub async fn get_by_month(
        &mut self,
        year: i32,
        from: Month,
        to: Month,
    ) -> Result<Vec<RetrieveDataWithCustomerAndPrice>, RetrieveDataRepositoryErr> {
        let conn = get_database_connection().await;

        let from_month = NaiveDate::from_ymd_opt(year, from.number_from_month(), 1)
            .unwrap()
            .and_hms_opt(1, 0, 0)
            .unwrap();

        let to_month = from_month.with_month(to.number_from_month()).unwrap();

        let data = RetrieveDataDb::find()
            .filter(retrieve_data_db::Column::Date.between(from_month, to_month))
            .find_also_related(price::Entity)
            .find_also_related(customer::Entity)
            .all(conn)
            .await
            .map_err(|_| RetrieveDataRepositoryErr::FailToGetByMonth)?;

        let retrieves_data = self.map_retrieve_data_db_to_final_model(data).await?;

        Ok(retrieves_data)
    }

    pub async fn get_by_year(
        &mut self,
        year: i32,
    ) -> Result<Vec<RetrieveDataWithCustomerAndPrice>, RetrieveDataRepositoryErr> {
        let conn = get_database_connection().await;

        let current_year = NaiveDate::from_ymd_opt(year, 1, 1)
            .unwrap()
            .and_hms_opt(1, 0, 0)
            .unwrap();

        let next_year = current_year.with_year(year + 1).unwrap();

        let data = RetrieveDataDb::find()
            .filter(retrieve_data_db::Column::Date.between(current_year, next_year))
            .find_also_related(price::Entity)
            .find_also_related(customer::Entity)
            .all(conn)
            .await
            .map_err(|_| RetrieveDataRepositoryErr::FailToGetByYear)?;

        let retrieves_data = self.map_retrieve_data_db_to_final_model(data).await?;

        Ok(retrieves_data)
    }

    pub async fn get_by_customer_id_and_month(
        &mut self,
        customer_id: i64,
        year: i32,
        from: Month,
        to: Month,
    ) -> Result<Vec<RetrieveDataWithCustomerAndPrice>, RetrieveDataRepositoryErr> {
        let conn = get_database_connection().await;

        let from_month = NaiveDate::from_ymd_opt(year, from.number_from_month(), 1)
            .unwrap()
            .and_hms_opt(1, 0, 0)
            .unwrap();

        let to_month = from_month.with_month(to.number_from_month()).unwrap();

        let data = RetrieveDataDb::find()
            .filter(retrieve_data_db::Column::CustomerId.eq(customer_id))
            .filter(retrieve_data_db::Column::Date.between(from_month, to_month))
            .find_also_related(price::Entity)
            .find_also_related(customer::Entity)
            .all(conn)
            .await
            .map_err(|_| RetrieveDataRepositoryErr::FailToGetByCustomerIdAndMonth)?;

        let retrieves_data = self.map_retrieve_data_db_to_final_model(data).await?;

        Ok(retrieves_data)
    }

    pub async fn get_by_customer_id_and_year(
        &mut self,
        customer_id: i64,
        year: i32,
    ) -> Result<Vec<RetrieveDataWithCustomerAndPrice>, RetrieveDataRepositoryErr> {
        let conn = get_database_connection().await;

        let current_year = NaiveDate::from_ymd_opt(year, 1, 1)
            .unwrap()
            .and_hms_opt(1, 0, 0)
            .unwrap();

        let next_year = current_year.with_year(year + 1).unwrap();

        let data = RetrieveDataDb::find()
            .filter(retrieve_data_db::Column::CustomerId.eq(customer_id))
            .filter(retrieve_data_db::Column::Date.between(current_year, next_year))
            .find_also_related(price::Entity)
            .find_also_related(customer::Entity)
            .all(conn)
            .await
            .map_err(|_| RetrieveDataRepositoryErr::FailToGetByCustomerIdAndYear)?;

        let retrieves_data = self.map_retrieve_data_db_to_final_model(data).await?;

        Ok(retrieves_data)
    }

    pub async fn create(
        &mut self,
        model: RetrieveDataCreateOrUpdate,
    ) -> Result<i64, BaseRepositoryErr> {
        let active_model = model.to_active_without_id();

        let result = RetrieveDataDb.create(active_model).await;

        match result {
            Ok(created_model) => Ok(created_model.retrieve_data_id),
            Err(_) => Err(BaseRepositoryErr::FailToCreate),
        }
    }

    pub async fn update(
        &mut self,
        model: RetrieveDataCreateOrUpdate,
    ) -> Result<RetrieveDataWithCustomerAndPrice, BaseRepositoryErr> {
        let active_model = model.to_active_with_id();
        let update_result = RetrieveDataDb.update_by_model(active_model).await;

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

                let retrieve_data = RetrieveDataWithCustomerAndPrice::with_price_and_customer(
                    model,
                    price.into(),
                    customer.into(),
                );

                Ok(retrieve_data)
            }
            Err(_) => Err(BaseRepositoryErr::FailToUpdate),
        }
    }
}

impl BaseRepository<RetrieveDataWithCustomerAndPrice> for RetrieveDataRepository {
    async fn create(
        &mut self,
        model: RetrieveDataWithCustomerAndPrice,
    ) -> Result<i64, BaseRepositoryErr> {
        let active_model = model.to_active_without_id();

        let result = RetrieveDataDb.create(active_model).await;

        match result {
            Ok(created_model) => Ok(created_model.retrieve_data_id),
            Err(_) => Err(BaseRepositoryErr::FailToCreate),
        }
    }

    async fn read(
        &mut self,
        id: i64,
    ) -> Result<Option<RetrieveDataWithCustomerAndPrice>, BaseRepositoryErr> {
        match RetrieveDataDb.get_by_id(id).await {
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

                let retrieve_data = RetrieveDataWithCustomerAndPrice::with_price_and_customer(
                    model,
                    price.into(),
                    customer.into(),
                );

                Ok(Some(retrieve_data))
            }
            Err(_) => Err(BaseRepositoryErr::FailToCreate),
        }
    }

    async fn update(
        &mut self,
        model: RetrieveDataWithCustomerAndPrice,
    ) -> Result<RetrieveDataWithCustomerAndPrice, BaseRepositoryErr> {
        let active_model = model.to_active_with_id();
        let update_result = RetrieveDataDb.update_by_model(active_model).await;

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

                let retrieve_data = RetrieveDataWithCustomerAndPrice::with_price_and_customer(
                    model,
                    price.into(),
                    customer.into(),
                );

                Ok(retrieve_data)
            }
            Err(_) => Err(BaseRepositoryErr::FailToUpdate),
        }
    }

    async fn delete(&mut self, id: i64) -> Result<u64, BaseRepositoryErr> {
        match RetrieveDataDb.delete_by_model_id(id).await {
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
