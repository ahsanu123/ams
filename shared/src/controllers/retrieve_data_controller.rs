use crate::{
    models::retrieve_data::{
        retrieve_data_create_or_update::RetrieveDataCreateOrUpdate,
        retrieve_data_with_customer_and_price::RetrieveDataWithCustomerAndPrice,
    },
    repositories::{RETRIEVE_DATA_REPO, base_repository_trait::BaseRepository},
};
use chrono::Month;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct RetrieveDataGetAllProps {
    customer_id: Option<i64>,
    #[ts(type = "number", optional)]
    start_month: Option<Month>,
    #[ts(type = "number", optional)]
    end_month: Option<Month>,
    year: Option<i32>,
}

pub trait RetrieveDataControllerTrait {
    async fn create_record(&mut self, data: RetrieveDataCreateOrUpdate) -> i64;
    async fn get_all(
        &mut self,
        props: RetrieveDataGetAllProps,
    ) -> Vec<RetrieveDataWithCustomerAndPrice>;
    async fn update(
        &mut self,
        data: RetrieveDataCreateOrUpdate,
    ) -> Option<RetrieveDataWithCustomerAndPrice>;
    async fn delete(&mut self, retrieve_data_id: i64) -> u64;
}

pub struct RetrieveDataController;

impl RetrieveDataControllerTrait for RetrieveDataController {
    async fn create_record(&mut self, data: RetrieveDataCreateOrUpdate) -> i64 {
        RETRIEVE_DATA_REPO
            .lock()
            .await
            .create(data)
            .await
            .unwrap_or_default()
    }

    async fn get_all(
        &mut self,
        props: RetrieveDataGetAllProps,
    ) -> Vec<RetrieveDataWithCustomerAndPrice> {
        match props {
            RetrieveDataGetAllProps {
                customer_id: Some(customer_id),
                start_month: None,
                end_month: None,
                year: None,
            } => RETRIEVE_DATA_REPO
                .lock()
                .await
                .get_by_customer_id(customer_id)
                .await
                .unwrap_or_default(),

            RetrieveDataGetAllProps {
                customer_id: None,
                start_month: Some(from),
                end_month: Some(to),
                year: Some(year),
            } => RETRIEVE_DATA_REPO
                .lock()
                .await
                .get_by_month(year, from, to)
                .await
                .unwrap_or_default(),

            RetrieveDataGetAllProps {
                customer_id: None,
                start_month: None,
                end_month: None,
                year: Some(year),
            } => RETRIEVE_DATA_REPO
                .lock()
                .await
                .get_by_year(year)
                .await
                .unwrap_or_default(),

            RetrieveDataGetAllProps {
                customer_id: Some(customer_id),
                start_month: None,
                end_month: None,
                year: Some(year),
            } => RETRIEVE_DATA_REPO
                .lock()
                .await
                .get_by_customer_id_and_year(customer_id, year)
                .await
                .unwrap_or_default(),

            RetrieveDataGetAllProps {
                customer_id: Some(customer_id),
                start_month: Some(from),
                end_month: Some(to),
                year: Some(year),
            } => RETRIEVE_DATA_REPO
                .lock()
                .await
                .get_by_customer_id_and_month(customer_id, year, from, to)
                .await
                .unwrap_or_default(),

            _ => Vec::<RetrieveDataWithCustomerAndPrice>::new(),
        }
    }

    async fn update(
        &mut self,
        data: RetrieveDataCreateOrUpdate,
    ) -> Option<RetrieveDataWithCustomerAndPrice> {
        RETRIEVE_DATA_REPO.lock().await.update(data).await.ok()
    }

    async fn delete(&mut self, retrieve_data_id: i64) -> u64 {
        RETRIEVE_DATA_REPO
            .lock()
            .await
            .delete(retrieve_data_id)
            .await
            .unwrap_or_default()
    }
}
