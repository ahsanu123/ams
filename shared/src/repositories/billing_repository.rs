use crate::{
    models::{
        billing::{Billing, BillingCreate, BillingUpdate},
        customer::Customer,
    },
    repositories::{
        base_repository_trait::{BaseRepository2, BaseRepositoryErr},
        generic_crud_repository::GenericCrudRepository,
    },
    sqls::billing::{create_billing, get_by_billing_id, get_by_customer_id, update_by_billing},
};
use ams_entity::prelude::Billing as BillingDb;
use ams_entity::prelude::Customer as CustomerDb;

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

        let results = get_by_customer_id::query(customer_id)
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
