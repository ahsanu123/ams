use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::{IntoParams, ToSchema};

use crate::{
    models::{
        balance::{BalanceCreateOrUpdateWithoutChangedValue, BalanceWithCustomer, TransactionType},
        billing::{Billing, BillingUpdate},
    },
    repositories::{BALANCE_REPO, base_repository_trait::BaseRepositoryWithCRUType},
};

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams, Clone, TS)]
#[into_params(parameter_in = Query)]
pub struct BalanceGetAllProps {
    customer_id: Option<i64>,

    year: Option<i32>,

    #[ts(type = "Date")]
    start_date: Option<NaiveDateTime>,

    #[ts(type = "Date")]
    end_date: Option<NaiveDateTime>,
}

#[derive(Serialize)]
pub enum BalanceControllerErr {
    FailToGetLatest,
    FailToGetAll,
    FailToCreate,
    UnknownQuery,
}

pub trait BalanceControllerTrait {
    fn get_all(
        &mut self,
        props: BalanceGetAllProps,
    ) -> impl Future<Output = Result<Vec<BalanceWithCustomer>, BalanceControllerErr>>;

    fn get_latest(
        &mut self,
        customer_id: i64,
    ) -> impl Future<Output = Result<BalanceWithCustomer, BalanceControllerErr>>;

    fn update(&mut self, data: BillingUpdate) -> impl Future<Output = Billing>;

    fn add_balance(
        &mut self,
        data: BalanceCreateOrUpdateWithoutChangedValue,
    ) -> impl Future<Output = Result<i64, BalanceControllerErr>>;
}

pub struct BalanceController;

impl BalanceControllerTrait for BalanceController {
    async fn add_balance(
        &mut self,
        mut data: BalanceCreateOrUpdateWithoutChangedValue,
    ) -> Result<i64, BalanceControllerErr> {
        let latest_balance = self.get_latest(data.customer_id).await;

        match latest_balance {
            Ok(latest_balance) => {
                let changed_value = if data.transaction_type == TransactionType::TopUp {
                    data.value
                } else {
                    -data.value
                };

                let value = if data.transaction_type == TransactionType::TopUp {
                    latest_balance.value + data.value
                } else {
                    latest_balance.value - data.value
                };

                let mut final_data = data.with_changed_value(changed_value);
                final_data.value = value;

                BALANCE_REPO
                    .lock()
                    .await
                    .create(final_data)
                    .await
                    .map_err(|_| BalanceControllerErr::FailToCreate)
            }
            Err(_) => {
                let final_data = data.with_changed_value(0.0);

                BALANCE_REPO
                    .lock()
                    .await
                    .create(final_data)
                    .await
                    .map_err(|_| BalanceControllerErr::FailToCreate)
            }
        }
    }

    async fn get_all(
        &mut self,
        props: BalanceGetAllProps,
    ) -> Result<Vec<BalanceWithCustomer>, BalanceControllerErr> {
        match props {
            BalanceGetAllProps {
                customer_id: Some(customer_id),
                start_date: None,
                end_date: None,
                year: None,
            } => BALANCE_REPO
                .lock()
                .await
                .get_by_customer_id(customer_id)
                .await
                .map_err(|_| BalanceControllerErr::FailToGetAll),

            BalanceGetAllProps {
                customer_id: Some(customer_id),
                start_date: None,
                end_date: None,
                year: Some(year),
            } => BALANCE_REPO
                .lock()
                .await
                .get_by_customer_id_and_year(customer_id, year)
                .await
                .map_err(|_| BalanceControllerErr::FailToGetAll),

            BalanceGetAllProps {
                customer_id: Some(customer_id),
                start_date: Some(start_month),
                end_date: Some(end_month),
                year: Some(year),
            } => BALANCE_REPO
                .lock()
                .await
                .get_by_customer_id_and_month_range(customer_id, start_month, end_month, year)
                .await
                .map_err(|_| BalanceControllerErr::FailToGetAll),

            BalanceGetAllProps {
                customer_id: None,
                start_date: None,
                end_date: None,
                year: Some(year),
            } => BALANCE_REPO
                .lock()
                .await
                .get_by_year(year)
                .await
                .map_err(|_| BalanceControllerErr::FailToGetAll),

            _ => Err(BalanceControllerErr::UnknownQuery),
        }
    }

    async fn get_latest(
        &mut self,
        customer_id: i64,
    ) -> Result<BalanceWithCustomer, BalanceControllerErr> {
        BALANCE_REPO
            .lock()
            .await
            .get_latest_by_customer_id(customer_id)
            .await
            .map_err(|_| BalanceControllerErr::FailToGetLatest)
    }

    async fn update(&mut self, data: BillingUpdate) -> Billing {
        todo!()
    }
}
