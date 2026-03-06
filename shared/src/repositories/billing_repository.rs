use std::collections::HashMap;

use crate::{
    models::{
        billing::{
            Billing, BillingCreate, BillingUpdate,
            billing_info::{BillingInfo, BillingInfoWithBalance},
            billing_with_retrieve_data::BillingWithRetrieveData,
        },
        customer::Customer,
        price::Price,
        retrieve_data::{
            RetrieveData, retrieve_data_with_customer_and_price::RetrieveDataWithCustomerAndPrice,
        },
    },
    repositories::{
        base_repository_trait::{BaseRepositoryErr, BaseRepositoryWithCRUType},
        database_connection::get_database_connection,
        generic_crud_repository::GenericCrudRepository,
    },
    sqls::billing::{
        create_billing, get_all_billing, get_billing_by_billing_id, get_billing_by_customer_id,
        get_billing_info_by_customer_id, get_billing_info_by_date,
        get_billing_info_by_date_and_customer_id, get_by_billing_id, get_by_customer_id,
        update_by_billing,
    },
};
use ams_entity::billing_retrieve_data as billing_retrieve_data_db;
use ams_entity::prelude::Billing as BillingDb;
use ams_entity::prelude::Customer as CustomerDb;
use ams_entity::prelude::Price as PriceDb;
use ams_entity::prelude::RetrieveData as RetrieveDataDb;
use ams_entity::retrieve_data as retrieve_data_db;
use chrono::{Datelike, NaiveDate, NaiveDateTime};
use itertools::Itertools;
use sea_orm::{
    ColumnTrait, EntityTrait, ExprTrait, JoinType, QueryFilter, QuerySelect, RelationTrait,
    prelude::Expr,
};

#[derive(Debug)]
pub enum BillingRepositoryErr {
    FailToGetByCustomerId,
    FailToConvertWithOtherData,
    FailToGetInfoByMonth,
    FailToGetInfoByYear,
    FailToGetInfoByCustomerIdAndMonth,
    FailToGetInfoByCustomerIdAndYear,
}

pub struct BillingRepository;

impl BillingRepository {
    pub async fn get_info_by_month(
        &mut self,
        month: NaiveDateTime,
    ) -> Result<Vec<BillingInfo>, BillingRepositoryErr> {
        let start_month = NaiveDate::from_ymd_opt(month.year(), month.month(), 1)
            .ok_or(BillingRepositoryErr::FailToGetInfoByMonth)?
            .and_hms_opt(1, 0, 0)
            .ok_or(BillingRepositoryErr::FailToGetInfoByMonth)?;

        let end_month = NaiveDate::from_ymd_opt(month.year(), month.month() + 1, 1)
            .ok_or(BillingRepositoryErr::FailToGetInfoByMonth)?
            .and_hms_opt(1, 0, 0)
            .ok_or(BillingRepositoryErr::FailToGetInfoByMonth)?;

        let billing_infos = get_billing_info_by_date::query(start_month, end_month)
            .await
            .map_err(|_| BillingRepositoryErr::FailToGetInfoByMonth)?;

        Ok(billing_infos)
    }

    pub async fn get_info_by_date_range(
        &mut self,
        from: NaiveDateTime,
        to: NaiveDateTime,
    ) -> Result<Vec<BillingInfo>, BillingRepositoryErr> {
        let billing_infos = get_billing_info_by_date::query(from, to)
            .await
            .map_err(|_| BillingRepositoryErr::FailToGetInfoByMonth)?;

        Ok(billing_infos)
    }

    pub async fn get_info_by_year(
        &mut self,
        year: i32,
    ) -> Result<Vec<BillingInfo>, BillingRepositoryErr> {
        let start_year = NaiveDate::from_ymd_opt(year, 1, 1)
            .ok_or(BillingRepositoryErr::FailToGetInfoByYear)?
            .and_hms_opt(1, 0, 0)
            .ok_or(BillingRepositoryErr::FailToGetInfoByYear)?;

        let end_year = NaiveDate::from_ymd_opt(year + 1, 1, 1)
            .ok_or(BillingRepositoryErr::FailToGetInfoByYear)?
            .and_hms_opt(1, 0, 0)
            .ok_or(BillingRepositoryErr::FailToGetInfoByYear)?;

        let billing_infos = get_billing_info_by_date::query(start_year, end_year)
            .await
            .map_err(|_| BillingRepositoryErr::FailToGetInfoByYear)?;

        Ok(billing_infos)
    }

    pub async fn get_info_by_customer_id_and_year(
        &mut self,
        customer_id: i64,
        year: i32,
    ) -> Result<Vec<BillingInfo>, BillingRepositoryErr> {
        let start_year = NaiveDate::from_ymd_opt(year, 1, 1)
            .ok_or(BillingRepositoryErr::FailToGetInfoByYear)?
            .and_hms_opt(1, 0, 0)
            .ok_or(BillingRepositoryErr::FailToGetInfoByYear)?;

        let end_year = NaiveDate::from_ymd_opt(year + 1, 1, 1)
            .ok_or(BillingRepositoryErr::FailToGetInfoByYear)?
            .and_hms_opt(1, 0, 0)
            .ok_or(BillingRepositoryErr::FailToGetInfoByYear)?;

        let billing_infos =
            get_billing_info_by_date_and_customer_id::query(start_year, end_year, customer_id)
                .await
                .map_err(|_| BillingRepositoryErr::FailToGetInfoByYear)?;

        Ok(billing_infos)
    }

    pub async fn get_info_by_customer_id_and_month(
        &mut self,
        customer_id: i64,
        month: NaiveDateTime,
    ) -> Result<Vec<BillingInfo>, BillingRepositoryErr> {
        let end_month = month
            .with_month(month.month() + 1)
            .ok_or(BillingRepositoryErr::FailToGetInfoByCustomerIdAndMonth)?;

        let billing_infos =
            get_billing_info_by_date_and_customer_id::query(month, end_month, customer_id)
                .await
                .map_err(|_| BillingRepositoryErr::FailToGetInfoByYear)?;

        Ok(billing_infos)
    }

    pub async fn get_info_by_customer_id_and_date_range(
        &mut self,
        customer_id: i64,
        from: NaiveDateTime,
        to: NaiveDateTime,
    ) -> Result<Vec<BillingInfo>, BillingRepositoryErr> {
        let billing_infos = get_billing_info_by_date_and_customer_id::query(from, to, customer_id)
            .await
            .map_err(|_| BillingRepositoryErr::FailToGetInfoByYear)?;

        Ok(billing_infos)
    }

    pub async fn get_info_by_customer_id(
        &mut self,
        customer_id: i64,
    ) -> Result<Vec<BillingInfo>, BillingRepositoryErr> {
        let billing_infos = get_billing_info_by_customer_id::query(customer_id)
            .await
            .map_err(|_| BillingRepositoryErr::FailToGetByCustomerId)?;

        Ok(billing_infos)
    }

    pub async fn get_by_customer_id(
        &mut self,
        customer_id: i64,
    ) -> Result<Vec<Billing>, BillingRepositoryErr> {
        let customer: Customer = CustomerDb
            .get_by_id(customer_id)
            .await
            .map_err(|_| BillingRepositoryErr::FailToGetByCustomerId)?
            .ok_or(BillingRepositoryErr::FailToGetByCustomerId)?
            .into();

        let results = get_by_customer_id::query(customer_id)
            .await
            .map_err(|_| BillingRepositoryErr::FailToGetByCustomerId)?;

        let results = results
            .iter()
            .map(|result| Billing::from_query_result(*result, customer.clone()))
            .collect::<Vec<Billing>>();

        Ok(results)
    }

    pub async fn get_by_billing_id(
        &mut self,
        billing_id: i64,
    ) -> Result<BillingInfoWithBalance, BillingRepositoryErr> {
        let billing_info_with_balance = get_billing_by_billing_id::query(billing_id)
            .await
            .map_err(|_| BillingRepositoryErr::FailToGetByCustomerId)?;

        Ok(billing_info_with_balance)
    }

    pub async fn get_billing_info_with_balance_by_customer_id(
        &mut self,
        customer_id: i64,
    ) -> Result<Vec<BillingInfoWithBalance>, BillingRepositoryErr> {
        let billing_info_with_balance = get_billing_by_customer_id::query(customer_id)
            .await
            .map_err(|_| BillingRepositoryErr::FailToGetByCustomerId)?;

        Ok(billing_info_with_balance)
    }

    pub async fn get_all_billing(
        &mut self,
    ) -> Result<Vec<BillingInfoWithBalance>, BillingRepositoryErr> {
        let billing_info_with_balance = get_all_billing::query()
            .await
            .map_err(|_| BillingRepositoryErr::FailToGetByCustomerId)?;

        Ok(billing_info_with_balance)
    }
}

impl BaseRepositoryWithCRUType for BillingRepository {
    type CreateType = BillingCreate;
    type ReturnType = Billing;
    type UpdateType = BillingUpdate;

    async fn create(&mut self, model: Self::CreateType) -> Result<i64, BaseRepositoryErr> {
        let query_result = create_billing::query(model.into())
            .await
            .map_err(|_| BaseRepositoryErr::FailToCreate)?;

        Ok(query_result as i64)
    }

    async fn read(&mut self, id: i64) -> Result<Option<Self::ReturnType>, BaseRepositoryErr> {
        let query_result = get_by_billing_id::query(id)
            .await
            .map_err(|_| BaseRepositoryErr::FailToRead)?
            .ok_or(BaseRepositoryErr::FailToRead)?;

        let customer: Customer = CustomerDb
            .get_by_id(query_result.customer_id)
            .await
            .map_err(|_| BaseRepositoryErr::FailToRead)?
            .ok_or(BaseRepositoryErr::FailToRead)?
            .into();

        let billing = Billing::from_query_result(query_result, customer);

        Ok(Some(billing))
    }

    async fn update(
        &mut self,
        model: Self::UpdateType,
    ) -> Result<Self::ReturnType, BaseRepositoryErr> {
        let query_result = update_by_billing::query(model.clone())
            .await
            .map_err(|_| BaseRepositoryErr::FailToUpdate)?;

        if query_result == 0 {
            return Err(BaseRepositoryErr::FailToUpdate);
        }

        self.read(model.billing_id)
            .await
            .map_err(|_| BaseRepositoryErr::FailToUpdate)?
            .ok_or(BaseRepositoryErr::FailToUpdate)
    }

    async fn delete(&mut self, id: i64) -> Result<u64, BaseRepositoryErr> {
        match BillingDb.delete_by_model_id(id).await {
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

#[cfg(test)]
mod test_billing_repository {

    use super::*;
    use crate::init_environment_variable;
    use chrono::{Local, NaiveDate};

    #[tokio::test]
    async fn test_get_by_customer_id() {
        init_environment_variable();
        let result = BillingRepository
            .get_by_customer_id(1)
            .await
            .expect("fail to query from statement with stored procedure");

        println!("{:#?}", result);
    }

    #[tokio::test]
    async fn test_create_billing_sp() {
        init_environment_variable();
        let billing = BillingCreate {
            customer_id: 3,
            from: NaiveDate::from_ymd_opt(2026, 2, 19)
                .unwrap()
                .and_hms_opt(1, 0, 0)
                .unwrap(),
            to: NaiveDate::from_ymd_opt(2026, 2, 24)
                .unwrap()
                .and_hms_opt(1, 0, 0)
                .unwrap(),
        };
        let result = BillingRepository
            .create(billing)
            .await
            .expect("fail to insert");

        assert!(result > 0);
        println!("{:#?}", result);
    }

    #[tokio::test]
    async fn test_update_billing_sp() {
        init_environment_variable();
        let billing = BillingUpdate {
            customer_id: 3,
            date: Local::now().naive_local(),
            from: NaiveDate::from_ymd_opt(2026, 2, 20)
                .unwrap()
                .and_hms_opt(1, 0, 0)
                .unwrap(),
            to: NaiveDate::from_ymd_opt(2026, 2, 22)
                .unwrap()
                .and_hms_opt(23, 0, 0)
                .unwrap(),
            billing_id: 16,
        };
        let result = BillingRepository
            .update(billing.clone())
            .await
            .expect("fail to update");

        println!("{:#?}", result);
        println!("{:#?}", billing.date);
    }

    #[tokio::test]
    async fn test_delete_billing_sp() {
        init_environment_variable();
        let result = BillingRepository.delete(16).await.expect("fail to delete");

        println!("{:#?}", result);
    }
}
