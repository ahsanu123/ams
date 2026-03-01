use crate::{
    models::{
        billing::{
            Billing, BillingCreate, BillingUpdate, billing_info::BillingInfo,
            billing_with_retrieve_data::BillingWithRetrieveData,
        },
        customer::Customer,
    },
    repositories::{
        base_repository_trait::{BaseRepositoryErr, BaseRepositoryWithCRUType},
        database_connection::get_database_connection,
        generic_crud_repository::GenericCrudRepository,
    },
    sqls::billing::{create_billing, get_by_billing_id, get_by_customer_id, update_by_billing},
};
use ams_entity::billing_retrieve_data as billing_retrieve_data_db;
use ams_entity::prelude::Billing as BillingDb;
use ams_entity::prelude::Customer as CustomerDb;
use ams_entity::prelude::Price as PriceDb;
use ams_entity::prelude::RetrieveData as RetrieveDataDb;
use ams_entity::retrieve_data as retrieve_data_db;
use chrono::NaiveDateTime;
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
        let conn = get_database_connection().await;

        let query = RetrieveDataDb::find()
            .select_only()
            .find_also_related(CustomerDb)
            .find_also_related(PriceDb)
            .expr_as(
                Expr::col(retrieve_data_db::Column::Amount).sum(),
                "total_amount",
            )
            .join(JoinType::Join, retrieve_data_db::Relation::Customer.def())
            .join(JoinType::Join, retrieve_data_db::Relation::Price.def())
            // .filter(retrieve_data_db::Column::Date.between(a, b))
            .all(conn)
            .await;
        todo!()
    }

    pub async fn get_info_by_year(
        &mut self,
        year: i32,
    ) -> Result<Vec<BillingInfo>, BillingRepositoryErr> {
        todo!()
    }

    pub async fn get_info_by_customer_id_and_year(
        &mut self,
        customer_id: i64,
        year: i32,
    ) -> Result<Vec<BillingInfo>, BillingRepositoryErr> {
        todo!()
    }

    pub async fn get_info_by_customer_id_and_month(
        &mut self,
        customer_id: i64,
        month: NaiveDateTime,
    ) -> Result<Vec<BillingInfo>, BillingRepositoryErr> {
        todo!()
    }

    pub async fn get_info_by_customer_id(
        &mut self,
        customer_id: i64,
    ) -> Result<Vec<BillingInfo>, BillingRepositoryErr> {
        todo!()
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

    pub async fn get_with_retrieve_data_by_customer_id(
        &mut self,
        customer_id: i64,
    ) -> Result<Vec<BillingWithRetrieveData>, BillingRepositoryErr> {
        let conn = get_database_connection().await;

        let customer: Customer = CustomerDb
            .get_by_id(customer_id)
            .await
            .map_err(|_| BillingRepositoryErr::FailToGetByCustomerId)?
            .ok_or(BillingRepositoryErr::FailToGetByCustomerId)?
            .into();

        let results = get_by_customer_id::query(customer_id)
            .await
            .map_err(|_| BillingRepositoryErr::FailToGetByCustomerId)?;

        let billing_ids = results
            .iter()
            .map(|data| data.billing_id)
            .collect::<Vec<i64>>();

        let rds = RetrieveDataDb::find()
            .join(
                JoinType::Join,
                retrieve_data_db::Relation::BillingRetrieveData.def(),
            )
            .filter(billing_retrieve_data_db::Column::BillingId.is_in(billing_ids))
            .all(conn)
            .await
            .map_err(|_| BillingRepositoryErr::FailToGetByCustomerId)?;
        // .iter()
        // .map(|data| data.into())
        // .collect::<Vec<RetrieveData>>();

        let results = results
            .iter()
            .map(|result| Billing::from_query_result(*result, customer.clone()))
            .collect::<Vec<Billing>>();

        // Ok(results)
        todo!()
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
