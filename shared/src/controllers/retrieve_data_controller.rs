use crate::{
    models::retrieve_data::{
        retrieve_data_create_or_update::{
            RetrieveDataCreate, RetrieveDataCreateOrUpdate, RetrieveDataCreateWithDate,
        },
        retrieve_data_with_customer_and_price::RetrieveDataWithCustomerAndPrice,
    },
    repositories::{PRICE_REPO, RETRIEVE_DATA_REPO, base_repository_trait::BaseRepository},
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RetrieveDataControllerErr {
    FailToCreate,
}

pub trait RetrieveDataControllerTrait {
    async fn create(&mut self, data: RetrieveDataCreate) -> Result<i64, RetrieveDataControllerErr>;

    async fn delete(&mut self, retrieve_data_id: i64) -> Result<u64, RetrieveDataControllerErr>;

    async fn create_wd(
        &mut self,
        data: RetrieveDataCreateWithDate,
    ) -> Result<i64, RetrieveDataControllerErr>;

    async fn get_all(
        &mut self,
        props: RetrieveDataGetAllProps,
    ) -> Result<Vec<RetrieveDataWithCustomerAndPrice>, RetrieveDataControllerErr>;

    async fn update(
        &mut self,
        data: RetrieveDataCreateOrUpdate,
    ) -> Result<Option<RetrieveDataWithCustomerAndPrice>, RetrieveDataControllerErr>;
}

pub struct RetrieveDataController;

impl RetrieveDataControllerTrait for RetrieveDataController {
    async fn create(&mut self, data: RetrieveDataCreate) -> Result<i64, RetrieveDataControllerErr> {
        let latest_price = PRICE_REPO
            .lock()
            .await
            .get_latest()
            .await
            .map_err(|_| RetrieveDataControllerErr::FailToCreate)?;

        let data =
            RetrieveDataCreateOrUpdate::create_ca(data.customer_id, data.amount, latest_price);

        let created_id = RETRIEVE_DATA_REPO
            .lock()
            .await
            .create(data)
            .await
            .map_err(|_| RetrieveDataControllerErr::FailToCreate)?;

        Ok(created_id)
    }

    async fn create_wd(
        &mut self,
        data: RetrieveDataCreateWithDate,
    ) -> Result<i64, RetrieveDataControllerErr> {
        let latest_price = PRICE_REPO
            .lock()
            .await
            .get_latest()
            .await
            .map_err(|_| RetrieveDataControllerErr::FailToCreate)?;

        let data = RetrieveDataCreateOrUpdate::create_cawd(
            data.customer_id,
            data.amount,
            latest_price,
            data.date,
        );

        let created_id = RETRIEVE_DATA_REPO
            .lock()
            .await
            .create(data)
            .await
            .map_err(|_| RetrieveDataControllerErr::FailToCreate)?;

        Ok(created_id)
    }

    async fn get_all(
        &mut self,
        props: RetrieveDataGetAllProps,
    ) -> Result<Vec<RetrieveDataWithCustomerAndPrice>, RetrieveDataControllerErr> {
        let result = match props {
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
        };

        Ok(result)
    }

    async fn update(
        &mut self,
        data: RetrieveDataCreateOrUpdate,
    ) -> Result<Option<RetrieveDataWithCustomerAndPrice>, RetrieveDataControllerErr> {
        todo!()
        // RETRIEVE_DATA_REPO
        //     .lock()
        //     .await
        //     .update(data)
        //     .await
        //     .map_err(|_| RetrieveDataControllerErr::FailToCreate)
    }

    async fn delete(&mut self, retrieve_data_id: i64) -> Result<u64, RetrieveDataControllerErr> {
        RETRIEVE_DATA_REPO
            .lock()
            .await
            .delete(retrieve_data_id)
            .await
            .map_err(|_| RetrieveDataControllerErr::FailToCreate)
    }
}
