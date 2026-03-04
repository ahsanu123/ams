use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::{IntoParams, ToSchema};

use crate::{
    models::billing::{Billing, BillingCreate, BillingUpdate, billing_info::BillingInfo},
    repositories::{BILLING_REPO, base_repository_trait::BaseRepositoryWithCRUType},
};

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams, Clone, TS)]
#[into_params(parameter_in = Query)]
#[ts(export)]
pub struct BillingGetAllProps {
    customer_id: Option<i64>,
    year: Option<i32>,

    #[ts(type = "Date")]
    start_date: Option<NaiveDateTime>,
    #[ts(type = "Date")]
    end_date: Option<NaiveDateTime>,
}
pub trait BillingControllerTrait {
    fn get_all(
        &mut self,
        props: BillingGetAllProps,
    ) -> impl Future<Output = Result<Vec<BillingInfo>, BillingControllerErr>>;

    fn create(&mut self, data: BillingCreate) -> impl Future<Output = i64>;

    fn update(&mut self, data: BillingUpdate) -> impl Future<Output = Billing>;
}

#[derive(Serialize)]
pub enum BillingControllerErr {
    FailToGetByYear,
}

pub struct BillingController;

impl BillingControllerTrait for BillingController {
    async fn get_all(
        &mut self,
        props: BillingGetAllProps,
    ) -> Result<Vec<BillingInfo>, BillingControllerErr> {
        match props {
            BillingGetAllProps {
                customer_id: None,
                start_date: None,
                end_date: None,
                year: Some(year),
            } => BILLING_REPO
                .lock()
                .await
                .get_info_by_year(year)
                .await
                .map_err(|_| BillingControllerErr::FailToGetByYear),
            _ => todo!(),
        }
    }

    async fn create(&mut self, data: BillingCreate) -> i64 {
        BILLING_REPO
            .lock()
            .await
            .create(data)
            .await
            .unwrap_or_default()
    }

    async fn update(&mut self, data: BillingUpdate) -> Billing {
        BILLING_REPO
            .lock()
            .await
            .update(data)
            .await
            .unwrap_or_default()
    }
}
