use chrono::NaiveDateTime;

use crate::{
    models::billing::{Billing, BillingCreate, BillingUpdate, BillingWithRetrieveData},
    repositories::{BILLING_REPO, base_repository_trait::BaseRepositoryWithCRUType},
};

pub struct BillingGetAllProps {
    customer_id: Option<i64>,
    start_date: Option<NaiveDateTime>,
    end_date: Option<NaiveDateTime>,
    year: Option<i32>,
}
pub trait BillingControllerTrait {
    async fn get_all(&mut self, props: BillingGetAllProps) -> Vec<BillingWithRetrieveData>;
    async fn create(&mut self, data: BillingCreate) -> i64;
    async fn update(&mut self, data: BillingUpdate) -> Billing;
}

pub struct BillingController;

impl BillingControllerTrait for BillingController {
    async fn get_all(&mut self, props: BillingGetAllProps) -> Vec<BillingWithRetrieveData> {
        match props {
            BillingGetAllProps {
                customer_id: Some(customer_id),
                start_date: None,
                end_date: None,
                year: None,
            } => todo!(),
            // BILLING_REPO
            //     .lock()
            //     .await
            //     .get_by_customer_id(customer_id)
            //     .await
            //     .unwrap_or_default(),
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
