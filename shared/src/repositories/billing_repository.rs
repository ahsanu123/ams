use crate::{
    models::{
        billing::{Billing, BillingCreate},
        customer::Customer,
        to_active_without_id_trait::ToActiveModel,
    },
    repositories::{
        base_repository_trait::{BaseRepository, BaseRepository2, BaseRepositoryErr},
        database_connection::get_database_connection,
        generic_crud_repository::GenericCrudRepository,
    },
    sqls::billing::{
        create_billing, get_by_billing_id_query_result, get_by_customer_id_query_result,
    },
};
use ams_entity::billing as billing_db;
use ams_entity::prelude::Billing as BillingDb;
use ams_entity::prelude::BillingRetrieveData as BillingRetrieveDataDb;
use ams_entity::prelude::Customer as CustomerDb;
use ams_entity::prelude::RetrieveData as RetrieveDataDb;
use ams_entity::retrieve_data as retrieve_data_db;
use ams_entity::{billing::Model as BillingModel, price};
use ams_entity::{billing_retrieve_data as billing_retrieve_data_db, customer};
use chrono::NaiveDateTime;
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, Order, QueryFilter, QueryOrder};

#[derive(Debug)]
pub enum BillingRepositoryErr {
    FailToGeyByCustomerId,
    FailToConvertWithOtherData,
}

pub struct BillingRepository;

impl BillingRepository {
    pub async fn get_by_customer_id(
        &mut self,
        customer_id: i64,
    ) -> Result<Vec<Billing>, BillingRepositoryErr> {
        let customer: Customer = CustomerDb
            .get_by_id(customer_id)
            .await
            .map_err(|_| BillingRepositoryErr::FailToGeyByCustomerId)?
            .ok_or(BillingRepositoryErr::FailToGeyByCustomerId)?
            .into();

        let results = get_by_customer_id_query_result::query(customer_id)
            .await
            .map_err(|_| BillingRepositoryErr::FailToGeyByCustomerId)?;

        let result = results
            .iter()
            .map(|result| Billing::from_query_result(*result, customer.clone()))
            .collect::<Vec<Billing>>();

        Ok(result)
    }
}

impl BaseRepository2 for BillingRepository {
    type CreateType = BillingCreate;
    type ReturnType = Billing;

    async fn create(&mut self, model: Self::CreateType) -> Result<i64, BaseRepositoryErr> {
        let query_result = create_billing::query(model.into())
            .await
            .map_err(|_| BaseRepositoryErr::FailToCreate)?;

        Ok(query_result as i64)
    }

    async fn read(&mut self, id: i64) -> Result<Option<Self::ReturnType>, BaseRepositoryErr> {
        let query_result = get_by_billing_id_query_result::query(id)
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
        model: Self::ReturnType,
    ) -> Result<Self::ReturnType, BaseRepositoryErr> {
        let active_model = model.to_active_with_id();
        let update_result = BillingDb.update_by_model(active_model).await;

        match update_result {
            Ok(model) => todo!(),
            Err(_) => Err(BaseRepositoryErr::FailToUpdate),
        }
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

    use crate::init_environment_variable;

    use super::*;
    use chrono::{Local, NaiveDate};
    use sea_orm::QueryTrait;

    #[test]
    fn test_get_by_customer_id() {
        let billing_query = BillingRetrieveDataDb::find()
            .find_also_related(BillingDb)
            .find_also_related(RetrieveDataDb)
            .filter(billing_db::Column::CustomerId.eq(1))
            .build(sea_orm::DatabaseBackend::Sqlite)
            .to_string();
        println!("{}", billing_query);
    }

    #[tokio::test]
    async fn test_get_by_customer_id_with_statement_sp() {
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
            date: Local::now().naive_local(),
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

        println!("{:#?}", result);
    }
}
