use crate::{
    models::{
        balance::Balance, balance_billing::BalanceBilling, billing::Billing, customer::Customer,
        to_active_without_id_trait::ToActiveModel,
    },
    repositories::{
        base_repository_trait::{BaseRepository, BaseRepositoryErr, BaseRepositoryWithCRUType},
        billing_repository::BillingRepository,
        database_connection::get_database_connection,
        generic_crud_repository::GenericCrudRepository,
    },
    sqls::billing::{
        get_by_billing_id, get_by_customer_id, get_by_year, get_by_year_and_customer_id,
    },
};
use ams_entity::balance as balance_db;
use ams_entity::balance_billing as balance_billing_db;
use ams_entity::billing as billing_db;
use ams_entity::prelude::Balance as BalanceDb;
use ams_entity::prelude::BalanceBilling as BalanceBillingDb;
use ams_entity::prelude::Billing as BillingDb;
use ams_entity::prelude::Customer as CustomerDb;
use chrono::NaiveDate;
use sea_orm::{
    ColumnTrait, EntityTrait, JoinType, QueryFilter, QuerySelect, Related, RelationTrait,
    TransactionTrait,
};

pub enum BalanceBillingRepositoryErr {
    FailToGetByCustomerId,
    FailToGetByYear,
}

pub struct BalanceBillingRepository;

impl BalanceBillingRepository {
    pub async fn get_by_customer_id(
        &mut self,
        customer_id: i64,
    ) -> Result<Vec<BalanceBilling>, BalanceBillingRepositoryErr> {
        let conn = get_database_connection().await;

        let balance_billings = BalanceBillingDb::find()
            .find_also_related(BillingDb)
            .find_also_related(BalanceDb)
            .find_also(BillingDb, CustomerDb)
            .join(JoinType::Join, balance_billing_db::Relation::Balance.def())
            .join(JoinType::Join, balance_billing_db::Relation::Billing.def())
            .join(JoinType::Join, balance_db::Relation::Customer.def())
            .join(JoinType::Join, billing_db::Relation::Customer.def())
            .filter(billing_db::Column::CustomerId.eq(customer_id))
            .filter(balance_db::Column::CustomerId.eq(customer_id))
            .all(conn)
            .await
            .map_err(|_| BalanceBillingRepositoryErr::FailToGetByCustomerId)?;

        let billing_by_customer_id_res = get_by_customer_id::query(customer_id)
            .await
            .map_err(|_| BalanceBillingRepositoryErr::FailToGetByCustomerId)?;

        let mut bbs = Vec::<BalanceBilling>::new();

        for bb in balance_billings {
            let customer: Customer =
                bb.3.ok_or(BalanceBillingRepositoryErr::FailToGetByCustomerId)?
                    .into();

            let balance_model =
                bb.2.ok_or(BalanceBillingRepositoryErr::FailToGetByCustomerId)?;

            let balance = Balance::with_customer(balance_model, customer.clone());

            let billing_query_res = billing_by_customer_id_res
                .iter()
                .find(|b| b.billing_id == bb.0.billing_id)
                .ok_or(BalanceBillingRepositoryErr::FailToGetByCustomerId)?;

            let billing = Billing::from_query_result(*billing_query_res, customer);

            let balance_billing = BalanceBilling::with_balance_and_billing(bb.0, balance, billing);

            bbs.push(balance_billing);
        }

        Ok(bbs)
    }

    pub async fn get_by_year_and_customer_id(
        &mut self,
        year: i32,
        customer_id: i64,
    ) -> Result<Vec<BalanceBilling>, BalanceBillingRepositoryErr> {
        let conn = get_database_connection().await;

        let start_year = NaiveDate::from_ymd_opt(year, 1, 1)
            .unwrap()
            .and_hms_opt(1, 0, 0)
            .unwrap();

        let end_year = NaiveDate::from_ymd_opt(year + 1, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();

        let balance_billings = BalanceBillingDb::find()
            .find_also_related(BillingDb)
            .find_also_related(BalanceDb)
            .find_also(BillingDb, CustomerDb)
            .join(JoinType::Join, balance_billing_db::Relation::Balance.def())
            .join(JoinType::Join, balance_billing_db::Relation::Billing.def())
            .join(JoinType::Join, balance_db::Relation::Customer.def())
            .join(JoinType::Join, billing_db::Relation::Customer.def())
            .filter(billing_db::Column::Date.between(start_year, end_year))
            .filter(balance_db::Column::Date.between(start_year, end_year))
            .filter(billing_db::Column::CustomerId.eq(customer_id))
            .filter(balance_db::Column::CustomerId.eq(customer_id))
            .all(conn)
            .await
            .map_err(|_| BalanceBillingRepositoryErr::FailToGetByCustomerId)?;

        let billing_by_year_and_customer_res =
            get_by_year_and_customer_id::query(start_year, end_year, customer_id)
                .await
                .map_err(|_| BalanceBillingRepositoryErr::FailToGetByCustomerId)?;

        let mut bbs = Vec::<BalanceBilling>::new();

        for bb in balance_billings {
            let customer: Customer =
                bb.3.ok_or(BalanceBillingRepositoryErr::FailToGetByCustomerId)?
                    .into();

            let balance_model =
                bb.2.ok_or(BalanceBillingRepositoryErr::FailToGetByCustomerId)?;

            let balance = Balance::with_customer(balance_model, customer.clone());

            let billing_query_res = billing_by_year_and_customer_res
                .iter()
                .find(|b| b.billing_id == bb.0.billing_id)
                .ok_or(BalanceBillingRepositoryErr::FailToGetByCustomerId)?;

            let billing = Billing::from_query_result(*billing_query_res, customer);

            let balance_billing = BalanceBilling::with_balance_and_billing(bb.0, balance, billing);

            bbs.push(balance_billing);
        }

        Ok(bbs)
    }

    pub async fn get_by_year(
        &mut self,
        year: i32,
    ) -> Result<Vec<BalanceBilling>, BalanceBillingRepositoryErr> {
        let conn = get_database_connection().await;

        let start_year = NaiveDate::from_ymd_opt(year, 1, 1)
            .unwrap()
            .and_hms_opt(1, 0, 0)
            .unwrap();

        let end_year = NaiveDate::from_ymd_opt(year + 1, 1, 1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();

        let balance_billings = BalanceBillingDb::find()
            .find_also_related(BillingDb)
            .find_also_related(BalanceDb)
            .find_also(BillingDb, CustomerDb)
            .join(JoinType::Join, balance_billing_db::Relation::Balance.def())
            .join(JoinType::Join, balance_billing_db::Relation::Billing.def())
            .join(JoinType::Join, balance_db::Relation::Customer.def())
            .join(JoinType::Join, billing_db::Relation::Customer.def())
            .filter(billing_db::Column::Date.between(start_year, end_year))
            .filter(balance_db::Column::Date.between(start_year, end_year))
            .all(conn)
            .await
            .map_err(|_| BalanceBillingRepositoryErr::FailToGetByCustomerId)?;

        let billing_by_year_res = get_by_year::query(start_year, end_year)
            .await
            .map_err(|_| BalanceBillingRepositoryErr::FailToGetByCustomerId)?;

        let mut bbs = Vec::<BalanceBilling>::new();

        for bb in balance_billings {
            let customer: Customer =
                bb.3.ok_or(BalanceBillingRepositoryErr::FailToGetByCustomerId)?
                    .into();

            let balance_model =
                bb.2.ok_or(BalanceBillingRepositoryErr::FailToGetByCustomerId)?;

            let balance = Balance::with_customer(balance_model, customer.clone());

            let billing_query_res = billing_by_year_res
                .iter()
                .find(|b| b.billing_id == bb.0.billing_id)
                .ok_or(BalanceBillingRepositoryErr::FailToGetByCustomerId)?;

            let billing = Billing::from_query_result(*billing_query_res, customer);

            let balance_billing = BalanceBilling::with_balance_and_billing(bb.0, balance, billing);

            bbs.push(balance_billing);
        }

        Ok(bbs)
    }
}

impl BaseRepository<BalanceBilling> for BalanceBillingRepository {
    async fn create(&mut self, model: BalanceBilling) -> Result<i64, BaseRepositoryErr> {
        let active_model = model.to_active_without_id();
        let result = BalanceBillingDb.create(active_model).await;

        match result {
            Ok(created_model) => Ok(created_model.balance_billing_id),
            Err(_) => Err(BaseRepositoryErr::FailToCreate),
        }
    }

    // NOTE:
    // not best but work
    async fn read(&mut self, id: i64) -> Result<Option<BalanceBilling>, BaseRepositoryErr> {
        let model = BalanceBillingDb
            .get_by_id(id)
            .await
            .map_err(|_| BaseRepositoryErr::FailToRead)?
            .ok_or(BaseRepositoryErr::FailToRead)?;

        let balance = BalanceDb
            .get_by_id(model.balance_id)
            .await
            .map_err(|_| BaseRepositoryErr::FailToRead)?
            .ok_or(BaseRepositoryErr::FailToRead)?;

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

        let balance_billing =
            BalanceBilling::with_balance_model_and_billing(model, balance, billing);

        Ok(Some(balance_billing))
    }

    async fn update(&mut self, model: BalanceBilling) -> Result<BalanceBilling, BaseRepositoryErr> {
        let active_model = model.to_active_with_id();
        let update_result = BalanceBillingDb.update_by_model(active_model).await;

        match update_result {
            Ok(model) => {
                let balance_billing = self
                    .read(model.balance_billing_id)
                    .await
                    .map_err(|_| BaseRepositoryErr::FailToUpdate)?
                    .ok_or(BaseRepositoryErr::FailToUpdate)?;
                Ok(balance_billing)
            }
            Err(_) => Err(BaseRepositoryErr::FailToUpdate),
        }
    }

    async fn delete(&mut self, id: i64) -> Result<u64, BaseRepositoryErr> {
        match BalanceBillingDb.delete_by_model_id(id).await {
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
mod test_balance_billing_repository {
    use sea_orm::{DatabaseBackend, QueryTrait};

    use super::*;
    #[test]
    fn test_join_balance_billing_get_by_customer_id() {
        let statement = BalanceBillingDb::find()
            .find_also_related(BillingDb)
            .find_also_related(BalanceDb)
            .find_also(CustomerDb, BillingDb)
            .join(JoinType::Join, balance_billing_db::Relation::Balance.def())
            .join(JoinType::Join, balance_billing_db::Relation::Billing.def())
            .join(JoinType::Join, balance_db::Relation::Customer.def())
            .join(JoinType::Join, billing_db::Relation::Customer.def())
            .filter(billing_db::Column::CustomerId.eq(1))
            .filter(balance_db::Column::CustomerId.eq(1))
            .build(DatabaseBackend::Sqlite);

        println!("{}", statement.to_string());
    }
}
