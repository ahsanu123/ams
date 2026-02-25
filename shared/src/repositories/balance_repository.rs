use crate::{
    models::{balance::Balance, customer::Customer, to_active_without_id_trait::ToActiveModel},
    repositories::{
        base_repository_trait::{BaseRepository, BaseRepositoryErr},
        database_connection::get_database_connection,
        generic_crud_repository::GenericCrudRepository,
    },
};

use ams_entity::balance as balance_db;
use ams_entity::prelude::Balance as BalanceDb;
use ams_entity::prelude::Customer as CustomerDb;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, TransactionTrait,
};

pub enum BalanceRepositoryErr {
    FailToGetByCustomerId,
    FailToUpdateMany,
}

pub struct BalanceRepository;

impl BalanceRepository {
    pub async fn get_by_customer_id(
        &mut self,
        customer_id: i64,
    ) -> Result<Vec<Balance>, BalanceRepositoryErr> {
        let conn = get_database_connection().await;

        let balance_models = BalanceDb::find()
            .filter(balance_db::Column::CustomerId.eq(customer_id))
            .find_also_related(CustomerDb)
            .all(conn)
            .await
            .map_err(|_| BalanceRepositoryErr::FailToGetByCustomerId)?;

        let mut balances = Vec::<Balance>::new();

        for bm in balance_models {
            let customer: Customer =
                bm.1.ok_or(BalanceRepositoryErr::FailToGetByCustomerId)?
                    .into();
            let balance = Balance::with_customer(bm.0, customer);

            balances.push(balance);
        }

        Ok(balances)
    }

    pub async fn update_many(&mut self, models: Vec<Balance>) -> Result<i64, BalanceRepositoryErr> {
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

impl BaseRepository<Balance> for BalanceRepository {
    async fn create(&mut self, model: Balance) -> Result<i64, BaseRepositoryErr> {
        let active_model = model.to_active_without_id();
        let result = BalanceDb.create(active_model).await;

        match result {
            Ok(created_model) => Ok(created_model.balance_id),
            Err(_) => Err(BaseRepositoryErr::FailToCreate),
        }
    }

    async fn read(&mut self, id: i64) -> Result<Option<Balance>, BaseRepositoryErr> {
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

                Ok(Some(Balance::with_customer(model, customer)))
            }
            Err(_) => Err(BaseRepositoryErr::FailToCreate),
        }
    }

    async fn update(&mut self, model: Balance) -> Result<Balance, BaseRepositoryErr> {
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

                Ok(Balance::with_customer(model, customer))
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
