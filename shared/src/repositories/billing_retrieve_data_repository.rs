use crate::{
    models::{
        billing_retrieve_data::BillingRetrieveData, to_active_without_id_trait::ToActiveModel,
    },
    repositories::{
        base_repository_trait::{BaseRepository, BaseRepositoryErr},
        database_connection::get_database_connection,
        generic_crud_repository::GenericCrudRepository,
    },
};
use ams_entity::billing_retrieve_data::Model as BillingRetrieveDataModel;
use ams_entity::prelude::BillingRetrieveData as BillingRetrieveDataDb;
use ams_entity::prelude::RetrieveData as RetrieveDataDb;
use ams_entity::retrieve_data as retrieve_data_db;
use ams_entity::{billing_retrieve_data as billing_retrieve_data_db, price};
use chrono::NaiveDateTime;
use sea_orm::{ColumnTrait, EntityTrait, Order, QueryFilter, QueryOrder};

pub enum BillingRetrieveDataRepositoryErr {
    FailToGetByBillingId,
    FailToGetRealated,
}

pub struct BillingRetrieveDataRepository;

impl BillingRetrieveDataRepository {
    async fn with_other_data(
        &mut self,
        model: BillingRetrieveDataModel,
    ) -> Result<BillingRetrieveData, BillingRetrieveDataRepositoryErr> {
        let conn = get_database_connection().await;

        let billing_retrieve_data = BillingRetrieveDataDb::find()
            .filter(billing_retrieve_data_db::Column::BillingId.eq(model.billing_id))
            .all(conn)
            .await
            .map_err(|_| BillingRetrieveDataRepositoryErr::FailToGetRealated)?;

        let all_retrieves_data_ids = billing_retrieve_data
            .iter()
            .map(|billing| billing.retrieve_data_id)
            .collect::<Vec<i64>>();

        let retrieves_data_with_price = RetrieveDataDb::find()
            .filter(retrieve_data_db::Column::RetrieveDataId.is_in(all_retrieves_data_ids))
            .find_also_related(price::Entity)
            .order_by(retrieve_data_db::Column::Date, Order::Asc)
            .all(conn)
            .await
            .map_err(|_| BillingRetrieveDataRepositoryErr::FailToGetRealated)?;

        let from = retrieves_data_with_price
            .iter()
            .map(|(data, _)| data.date)
            .collect::<Vec<NaiveDateTime>>()
            .first()
            .ok_or(BillingRetrieveDataRepositoryErr::FailToGetRealated)?
            .clone();

        let to = retrieves_data_with_price
            .iter()
            .map(|(data, _)| data.date)
            .collect::<Vec<NaiveDateTime>>()
            .last()
            .ok_or(BillingRetrieveDataRepositoryErr::FailToGetRealated)?
            .clone();

        let bill = retrieves_data_with_price
            .iter()
            .map(|(data, price)| data.amount as f64 * price.as_ref().unwrap().value as f64)
            .sum::<f64>();

        let amount = retrieves_data_with_price
            .iter()
            .map(|(data, _)| data.amount)
            .sum::<i64>();

        Ok(BillingRetrieveData {
            billing_retrieve_data_id: model.billing_retrieve_data_id,
            billing_id: model.billing_id,
            retrieve_data_id: model.retrieve_data_id,
            from,
            to,
            bill,
            amount,
        })
    }

    pub async fn get_by_billing_id(
        &mut self,
        billing_id: i64,
    ) -> Result<BillingRetrieveData, BillingRetrieveDataRepositoryErr> {
        let conn = get_database_connection().await;

        let billing_retrieve_datas = BillingRetrieveDataDb::find()
            .filter(billing_retrieve_data_db::Column::BillingId.eq(billing_id))
            .all(conn)
            .await
            .map_err(|_| BillingRetrieveDataRepositoryErr::FailToGetByBillingId)?;

        let first_billing_retrieve_data = billing_retrieve_datas
            .first()
            .ok_or(BillingRetrieveDataRepositoryErr::FailToGetByBillingId)?;

        let all_retrieves_data_ids = billing_retrieve_datas
            .iter()
            .map(|billing| billing.retrieve_data_id)
            .collect::<Vec<i64>>();

        let retrieves_data_with_price = RetrieveDataDb::find()
            .filter(retrieve_data_db::Column::RetrieveDataId.is_in(all_retrieves_data_ids))
            .find_also_related(price::Entity)
            .order_by(retrieve_data_db::Column::Date, Order::Asc)
            .all(conn)
            .await
            .map_err(|_| BillingRetrieveDataRepositoryErr::FailToGetRealated)?;

        let from = retrieves_data_with_price
            .iter()
            .map(|(data, _)| data.date)
            .collect::<Vec<NaiveDateTime>>()
            .first()
            .ok_or(BillingRetrieveDataRepositoryErr::FailToGetRealated)?
            .clone();

        let to = retrieves_data_with_price
            .iter()
            .map(|(data, _)| data.date)
            .collect::<Vec<NaiveDateTime>>()
            .last()
            .ok_or(BillingRetrieveDataRepositoryErr::FailToGetRealated)?
            .clone();

        let bill = retrieves_data_with_price
            .iter()
            .map(|(data, price)| data.amount as f64 * price.as_ref().unwrap().value as f64)
            .sum::<f64>();

        let amount = retrieves_data_with_price
            .iter()
            .map(|(data, _)| data.amount)
            .sum::<i64>();

        Ok(BillingRetrieveData {
            billing_retrieve_data_id: first_billing_retrieve_data.billing_retrieve_data_id,
            billing_id: first_billing_retrieve_data.billing_id,
            retrieve_data_id: first_billing_retrieve_data.retrieve_data_id,
            from,
            to,
            bill,
            amount,
        })
    }
}

impl BaseRepository<BillingRetrieveData> for BillingRetrieveDataRepository {
    async fn create(&mut self, model: BillingRetrieveData) -> Result<i64, BaseRepositoryErr> {
        let active_model = model.to_active_without_id();
        let result = BillingRetrieveDataDb.create(active_model).await;

        match result {
            Ok(created_model) => Ok(created_model.retrieve_data_id),
            Err(_) => Err(BaseRepositoryErr::FailToCreate),
        }
    }

    async fn read(&mut self, id: i64) -> Result<Option<BillingRetrieveData>, BaseRepositoryErr> {
        match BillingRetrieveDataDb.get_by_id(id).await {
            Ok(model) => {
                let model = model.ok_or(BaseRepositoryErr::FailToRead)?;

                let billing_retrieve_data = self
                    .with_other_data(model)
                    .await
                    .map_err(|_| BaseRepositoryErr::FailToRead)?;

                Ok(Some(billing_retrieve_data))
            }
            Err(_) => Err(BaseRepositoryErr::FailToCreate),
        }
    }

    async fn update(
        &mut self,
        model: BillingRetrieveData,
    ) -> Result<BillingRetrieveData, BaseRepositoryErr> {
        let active_model = model.to_active_with_id();
        let update_result = BillingRetrieveDataDb.update_by_model(active_model).await;

        match update_result {
            Ok(model) => {
                let data = self
                    .with_other_data(model)
                    .await
                    .map_err(|_| BaseRepositoryErr::FailToUpdate)?;

                Ok(data)
            }

            Err(_) => Err(BaseRepositoryErr::FailToUpdate),
        }
    }

    async fn delete(&mut self, id: i64) -> Result<u64, BaseRepositoryErr> {
        match BillingRetrieveDataDb.delete_by_model_id(id).await {
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
