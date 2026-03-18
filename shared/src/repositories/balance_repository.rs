use crate::{
    models::{
        balance::{BalanceCreateOrUpdate, BalanceWithCustomer},
        customer::Customer,
        to_active_model_trait::ToActiveModel,
    },
    repositories::{
        base_repository_trait::{BaseRepositoryErr, BaseRepositoryWithCRUType},
        database_connection::get_database_connection,
        generic_crud_repository::GenericCrudRepository,
    },
};

use ams_entity::balance as balance_db;
use ams_entity::prelude::Balance as BalanceDb;
use ams_entity::prelude::Customer as CustomerDb;
use chrono::{Datelike, NaiveDate, NaiveDateTime};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, Order, QueryFilter, QueryOrder,
    TransactionTrait,
};

pub enum BalanceRepositoryErr {
    FailToGetByCustomerId,
    FailToGetByCustomerIdAndYear,
    FailToUpdateMany,
}

pub struct BalanceRepository;

impl BalanceRepository {
    pub async fn get_latest_by_customer_id(
        &mut self,
        customer_id: i64,
    ) -> Result<BalanceWithCustomer, BalanceRepositoryErr> {
        let conn = get_database_connection().await;

        let (balance_model, may_customer) = BalanceDb::find()
            .filter(balance_db::Column::CustomerId.eq(customer_id))
            .find_also_related(CustomerDb)
            .order_by(balance_db::Column::Date, Order::Desc)
            .one(conn)
            .await
            .map_err(|_| BalanceRepositoryErr::FailToGetByCustomerId)?
            .ok_or(BalanceRepositoryErr::FailToGetByCustomerId)?;

        let customer: Customer = may_customer
            .ok_or(BalanceRepositoryErr::FailToGetByCustomerId)?
            .into();

        Ok(BalanceWithCustomer::with_customer(balance_model, customer))
    }

    pub async fn get_by_customer_id(
        &mut self,
        customer_id: i64,
    ) -> Result<Vec<BalanceWithCustomer>, BalanceRepositoryErr> {
        let conn = get_database_connection().await;

        let balance_models = BalanceDb::find()
            .filter(balance_db::Column::CustomerId.eq(customer_id))
            .find_also_related(CustomerDb)
            .order_by(balance_db::Column::Date, Order::Desc)
            .all(conn)
            .await
            .map_err(|_| BalanceRepositoryErr::FailToGetByCustomerId)?;

        let mut balances = Vec::<BalanceWithCustomer>::new();

        for bm in balance_models {
            let customer: Customer =
                bm.1.ok_or(BalanceRepositoryErr::FailToGetByCustomerId)?
                    .into();
            let balance = BalanceWithCustomer::with_customer(bm.0, customer);

            balances.push(balance);
        }

        Ok(balances)
    }

    pub async fn get_by_year(
        &mut self,
        year: i32,
    ) -> Result<Vec<BalanceWithCustomer>, BalanceRepositoryErr> {
        let conn = get_database_connection().await;

        let start_year = NaiveDate::from_ymd_opt(year, 1, 1)
            .ok_or(BalanceRepositoryErr::FailToGetByCustomerIdAndYear)?;

        let end_year = start_year
            .with_year(year + 1)
            .ok_or(BalanceRepositoryErr::FailToGetByCustomerIdAndYear)?;

        let balance_models = BalanceDb::find()
            .filter(balance_db::Column::Date.between(start_year, end_year))
            .find_also_related(CustomerDb)
            .order_by(balance_db::Column::Date, Order::Desc)
            .all(conn)
            .await
            .map_err(|_| BalanceRepositoryErr::FailToGetByCustomerIdAndYear)?;

        let mut balances = Vec::<BalanceWithCustomer>::new();

        for bm in balance_models {
            let customer: Customer =
                bm.1.ok_or(BalanceRepositoryErr::FailToGetByCustomerIdAndYear)?
                    .into();
            let balance = BalanceWithCustomer::with_customer(bm.0, customer);

            balances.push(balance);
        }

        Ok(balances)
    }

    pub async fn get_by_customer_id_and_year(
        &mut self,
        customer_id: i64,
        year: i32,
    ) -> Result<Vec<BalanceWithCustomer>, BalanceRepositoryErr> {
        let conn = get_database_connection().await;

        let start_year = NaiveDate::from_ymd_opt(year, 1, 1)
            .ok_or(BalanceRepositoryErr::FailToGetByCustomerIdAndYear)?;

        let end_year = start_year
            .with_year(year + 1)
            .ok_or(BalanceRepositoryErr::FailToGetByCustomerIdAndYear)?;

        let balance_models = BalanceDb::find()
            .filter(balance_db::Column::CustomerId.eq(customer_id))
            .filter(balance_db::Column::Date.between(start_year, end_year))
            .find_also_related(CustomerDb)
            .order_by(balance_db::Column::Date, Order::Desc)
            .all(conn)
            .await
            .map_err(|_| BalanceRepositoryErr::FailToGetByCustomerIdAndYear)?;

        let mut balances = Vec::<BalanceWithCustomer>::new();

        for bm in balance_models {
            let customer: Customer =
                bm.1.ok_or(BalanceRepositoryErr::FailToGetByCustomerIdAndYear)?
                    .into();
            let balance = BalanceWithCustomer::with_customer(bm.0, customer);

            balances.push(balance);
        }

        Ok(balances)
    }

    pub async fn get_by_customer_id_and_month_range(
        &mut self,
        customer_id: i64,
        start_month: NaiveDateTime,
        end_month: NaiveDateTime,
        year: i32,
    ) -> Result<Vec<BalanceWithCustomer>, BalanceRepositoryErr> {
        let conn = get_database_connection().await;

        let from = NaiveDate::from_ymd_opt(year, start_month.month(), 1)
            .ok_or(BalanceRepositoryErr::FailToGetByCustomerIdAndYear)?;

        let to = NaiveDate::from_ymd_opt(year, end_month.month(), 1)
            .ok_or(BalanceRepositoryErr::FailToGetByCustomerIdAndYear)?;

        let balance_models = BalanceDb::find()
            .filter(balance_db::Column::CustomerId.eq(customer_id))
            .filter(balance_db::Column::Date.between(from, to))
            .find_also_related(CustomerDb)
            .order_by(balance_db::Column::Date, Order::Desc)
            .all(conn)
            .await
            .map_err(|_| BalanceRepositoryErr::FailToGetByCustomerIdAndYear)?;

        let mut balances = Vec::<BalanceWithCustomer>::new();

        for bm in balance_models {
            let customer: Customer =
                bm.1.ok_or(BalanceRepositoryErr::FailToGetByCustomerIdAndYear)?
                    .into();
            let balance = BalanceWithCustomer::with_customer(bm.0, customer);

            balances.push(balance);
        }

        Ok(balances)
    }

    pub async fn update_many(
        &mut self,
        models: Vec<BalanceWithCustomer>,
    ) -> Result<i64, BalanceRepositoryErr> {
        let conn = get_database_connection().await;

        let mut updated_count: i64 = 0;

        let transaction = conn
            .begin()
            .await
            .map_err(|_| BalanceRepositoryErr::FailToGetByCustomerId)?;

        for model in models {
            let active_model = model.to_active_with_id();

            let _ = active_model
                .update(&transaction)
                .await
                .map_err(|_| BalanceRepositoryErr::FailToUpdateMany)?;

            updated_count += 1;
        }

        transaction
            .commit()
            .await
            .map_err(|_| BalanceRepositoryErr::FailToGetByCustomerId)?;

        Ok(updated_count)
    }
}

impl BaseRepositoryWithCRUType for BalanceRepository {
    type CreateType = BalanceCreateOrUpdate;
    type ReturnType = BalanceWithCustomer;
    type UpdateType = BalanceCreateOrUpdate;

    async fn create(&mut self, model: BalanceCreateOrUpdate) -> Result<i64, BaseRepositoryErr> {
        let active_model = model.to_active_without_id();
        let result = BalanceDb.create(active_model).await;

        match result {
            Ok(created_model) => Ok(created_model.balance_id),
            Err(_) => Err(BaseRepositoryErr::FailToCreate),
        }
    }

    async fn read(&mut self, id: i64) -> Result<Option<BalanceWithCustomer>, BaseRepositoryErr> {
        let conn = get_database_connection().await;
        match BalanceDb.get_by_id(id).await {
            Ok(model) => {
                let model = model.ok_or(BaseRepositoryErr::FailToRead)?;
                let customer: Customer = model
                    .find_related(CustomerDb)
                    .one(conn)
                    .await
                    .map_err(|_| BaseRepositoryErr::FailToRead)?
                    .ok_or(BaseRepositoryErr::FailToRead)?
                    .into();

                Ok(Some(BalanceWithCustomer::with_customer(model, customer)))
            }
            Err(_) => Err(BaseRepositoryErr::FailToCreate),
        }
    }

    async fn update(
        &mut self,
        model: BalanceCreateOrUpdate,
    ) -> Result<BalanceWithCustomer, BaseRepositoryErr> {
        let conn = get_database_connection().await;

        let active_model = model.to_active_with_id();
        let update_result = BalanceDb.update_by_model(active_model).await;

        match update_result {
            Ok(model) => {
                let customer: Customer = model
                    .find_related(CustomerDb)
                    .one(conn)
                    .await
                    .map_err(|_| BaseRepositoryErr::FailToRead)?
                    .ok_or(BaseRepositoryErr::FailToRead)?
                    .into();

                Ok(BalanceWithCustomer::with_customer(model, customer))
            }
            Err(_) => Err(BaseRepositoryErr::FailToUpdate),
        }
    }

    async fn delete(&mut self, id: i64) -> Result<u64, BaseRepositoryErr> {
        match BalanceDb.delete_by_model_id(id).await {
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
