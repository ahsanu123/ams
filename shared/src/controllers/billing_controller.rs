use chrono::{Local, NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::{IntoParams, ToSchema};

use crate::{
    controllers::{BALANCE_CONTROLLER, balance_controller::BalanceControllerTrait},
    models::{
        balance::{BalanceCreateOrUpdateWithoutChangedValue, TransactionType},
        balance_billing::BalanceBillingCreateOrUpdate,
        billing::{
            Billing, BillingCreate, BillingUpdate,
            billing_info::{BillingInfo, BillingInfoWithBalance},
        },
    },
    repositories::{
        BALANCE_BILLING_REPO, BILLING_REPO, base_repository_trait::BaseRepositoryWithCRUType,
    },
};

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams, Clone, TS)]
#[into_params(parameter_in = Query)]
#[ts(export)]
pub struct BillingInfoGetAllProps {
    customer_id: Option<i64>,
    year: Option<i32>,
    month: Option<i32>,

    #[ts(type = "Date")]
    from: Option<NaiveDateTime>,
    #[ts(type = "Date")]
    to: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams, Clone, TS)]
#[into_params(parameter_in = Query)]
#[ts(export)]
pub struct BillingGetByProps {
    customer_id: Option<i64>,
    billing_id: Option<i64>,
}

pub trait BillingControllerTrait {
    fn get_all_billing_info(
        &mut self,
        props: BillingInfoGetAllProps,
    ) -> impl Future<Output = Result<Vec<BillingInfo>, BillingControllerErr>>;

    fn get_all_billing(
        &mut self,
    ) -> impl Future<Output = Result<Vec<BillingInfo>, BillingControllerErr>>;

    fn get_by(
        &mut self,
        props: BillingGetByProps,
    ) -> impl Future<Output = Result<Vec<BillingInfoWithBalance>, BillingControllerErr>>;

    fn create(
        &mut self,
        data: BillingCreate,
    ) -> impl Future<Output = Result<BillingInfo, BillingControllerErr>>;

    fn update(&mut self, data: BillingUpdate) -> impl Future<Output = Billing>;
}

#[derive(Serialize)]
pub enum BillingControllerErr {
    FailToGetByYear,
    FailToGetByBillingId,
    FailToCreate,
    UnknownQuery,
}

pub struct BillingController;

impl BillingControllerTrait for BillingController {
    async fn get_all_billing_info(
        &mut self,
        props: BillingInfoGetAllProps,
    ) -> Result<Vec<BillingInfo>, BillingControllerErr> {
        match props {
            BillingInfoGetAllProps {
                year: Some(year),
                customer_id: None,
                month: None,
                from: None,
                to: None,
            } => BILLING_REPO
                .lock()
                .await
                .get_info_by_year(year)
                .await
                .map_err(|_| BillingControllerErr::FailToGetByYear),

            BillingInfoGetAllProps {
                year: Some(year),
                customer_id: Some(customer_id),
                month: None,
                from: None,
                to: None,
            } => BILLING_REPO
                .lock()
                .await
                .get_info_by_customer_id_and_year(customer_id, year)
                .await
                .map_err(|_| BillingControllerErr::FailToGetByYear),

            BillingInfoGetAllProps {
                year: Some(year),
                customer_id: None,
                month: Some(month),
                from: None,
                to: None,
            } => BILLING_REPO
                .lock()
                .await
                .get_info_by_month(
                    NaiveDate::from_ymd_opt(year, month as u32, 1)
                        .ok_or(BillingControllerErr::FailToGetByYear)?
                        .and_hms_opt(1, 0, 0)
                        .ok_or(BillingControllerErr::FailToGetByYear)?,
                )
                .await
                .map_err(|_| BillingControllerErr::FailToGetByYear),

            BillingInfoGetAllProps {
                year: Some(year),
                customer_id: Some(customer_id),
                month: Some(month),
                from: None,
                to: None,
            } => {
                let month = NaiveDate::from_ymd_opt(year, month as u32, 1)
                    .ok_or(BillingControllerErr::FailToGetByYear)?
                    .and_hms_opt(1, 0, 0)
                    .ok_or(BillingControllerErr::FailToGetByYear)?;

                BILLING_REPO
                    .lock()
                    .await
                    .get_info_by_customer_id_and_month(customer_id, month)
                    .await
                    .map_err(|_| BillingControllerErr::FailToGetByYear)
            }

            BillingInfoGetAllProps {
                year: None,
                customer_id: Some(customer_id),
                month: None,
                from: None,
                to: None,
            } => BILLING_REPO
                .lock()
                .await
                .get_info_by_customer_id(customer_id)
                .await
                .map_err(|_| BillingControllerErr::FailToGetByYear),

            BillingInfoGetAllProps {
                year: None,
                customer_id: None,
                month: None,
                from: Some(from),
                to: Some(to),
            } => BILLING_REPO
                .lock()
                .await
                .get_info_by_date_range(from, to)
                .await
                .map_err(|_| BillingControllerErr::FailToGetByYear),

            BillingInfoGetAllProps {
                year: None,
                customer_id: Some(customer_id),
                month: None,
                from: Some(from),
                to: Some(to),
            } => BILLING_REPO
                .lock()
                .await
                .get_info_by_customer_id_and_date_range(customer_id, from, to)
                .await
                .map_err(|_| BillingControllerErr::FailToGetByYear),

            _ => Err(BillingControllerErr::UnknownQuery),
        }
    }

    async fn create(&mut self, data: BillingCreate) -> Result<BillingInfo, BillingControllerErr> {
        let info = self
            .get_all_billing_info(BillingInfoGetAllProps {
                customer_id: Some(data.customer_id),
                year: None,
                month: None,
                from: Some(data.from),
                to: Some(data.to),
            })
            .await?
            .first()
            .ok_or(BillingControllerErr::FailToCreate)?
            .clone();

        let balance_id = BALANCE_CONTROLLER
            .lock()
            .await
            .add_balance(BalanceCreateOrUpdateWithoutChangedValue {
                balance_id: 0,
                customer_id: data.customer_id,
                value: info.unpaid_bill,
                date: Local::now().naive_local(),
                transaction_type: TransactionType::Pay,
            })
            .await
            .map_err(|_| BillingControllerErr::FailToCreate)?;

        let billing_id = BILLING_REPO
            .lock()
            .await
            .create(data.clone())
            .await
            .map_err(|_| BillingControllerErr::FailToCreate)?;

        let balance_billing_id = BALANCE_BILLING_REPO
            .lock()
            .await
            .create(BalanceBillingCreateOrUpdate {
                balance_billing_id: 0,
                balance_id,
                billing_id,
            })
            .await
            .map_err(|_| BillingControllerErr::FailToCreate)?;

        if balance_billing_id == 0 {
            return Err(BillingControllerErr::FailToCreate);
        }

        self.get_all_billing_info(BillingInfoGetAllProps {
            customer_id: Some(data.customer_id),
            year: None,
            month: None,
            from: Some(data.from),
            to: Some(data.to),
        })
        .await?
        .first()
        .ok_or(BillingControllerErr::FailToCreate)
        .cloned()
    }

    async fn update(&mut self, data: BillingUpdate) -> Billing {
        BILLING_REPO
            .lock()
            .await
            .update(data)
            .await
            .unwrap_or_default()
    }

    async fn get_all_billing(&mut self) -> Result<Vec<BillingInfo>, BillingControllerErr> {
        todo!()
    }

    async fn get_by(
        &mut self,
        props: BillingGetByProps,
    ) -> Result<Vec<BillingInfoWithBalance>, BillingControllerErr> {
        match props {
            BillingGetByProps {
                customer_id: None,
                billing_id: Some(billing_id),
            } => {
                let result = BILLING_REPO
                    .lock()
                    .await
                    .get_by_billing_id(billing_id)
                    .await
                    .map_err(|_| BillingControllerErr::FailToGetByBillingId)?;

                Ok([result].to_vec())
            }

            BillingGetByProps {
                customer_id: Some(customer_id),
                billing_id: None,
            } => BILLING_REPO
                .lock()
                .await
                .get_billing_info_with_balance_by_customer_id(customer_id)
                .await
                .map_err(|_| BillingControllerErr::FailToGetByBillingId),

            _ => Err(BillingControllerErr::UnknownQuery),
        }
    }

    // async fn get_by_billing_id(
    //     &mut self,
    //     billing_id: i64,
    // ) -> Result<BillingInfoWithBalance, BillingControllerErr> {
    //     BILLING_REPO
    //         .lock()
    //         .await
    //         .get_by_billing_id(billing_id)
    //         .await
    //         .map_err(|_| BillingControllerErr::FailToGetByBillingId)
    // }
    //
    // async fn get_by_customer_id(
    //     &mut self,
    //     customer_id: i64,
    // ) -> Result<Vec<BillingInfoWithBalance>, BillingControllerErr> {
    //     BILLING_REPO
    //         .lock()
    //         .await
    //         .get_billing_info_with_balance_by_customer_id(customer_id)
    //         .await
    //         .map_err(|_| BillingControllerErr::FailToGetByBillingId)
    // }
}
