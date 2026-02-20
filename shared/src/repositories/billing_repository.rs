use crate::{
    models::{
        billing::Billing, customer::Customer, retrieve_data::RetrieveData,
        to_active_without_id_trait::ToActiveModel,
    },
    repositories::{
        base_repository_trait::{BaseRepository, BaseRepositoryErr},
        database_connection::get_database_connection,
        generic_crud_repository::GenericCrudRepository,
    },
};
use ams_entity::billing as billing_db;
use ams_entity::billing_retrieve_data::Model as BillingRetrieveDataModel;
use ams_entity::prelude::Billing as BillingDb;
use ams_entity::prelude::BillingRetrieveData as BillingRetrieveDataDb;
use ams_entity::prelude::RetrieveData as RetrieveDataDb;
use ams_entity::retrieve_data as retrieve_data_db;
use ams_entity::retrieve_data::Model as RetrieveDataModel;
use ams_entity::{billing::Model as BillingModel, price};
use ams_entity::{billing_retrieve_data as billing_retrieve_data_db, customer};
use chrono::NaiveDateTime;
use sea_orm::{
    ColumnTrait, DatabaseBackend, EntityTrait, ExprTrait, JoinType, ModelTrait, Order, QueryFilter,
    QueryOrder, QuerySelect, RelationTrait, Statement, prelude::Expr, sea_query::IntoCondition,
    sqlx::types::uuid::Version,
};

pub enum BillingRepositoryErr {
    FailToGeyByCustomerId,
    FailToConvertWithOtherData,
}

pub struct BillingRepository;

impl BillingRepository {
    async fn with_other_data(
        &mut self,
        model: BillingModel,
    ) -> Result<Billing, BillingRepositoryErr> {
        let conn = get_database_connection().await;

        let customer: Customer = model
            .find_related(customer::Entity)
            .one(conn)
            .await
            .map_err(|_| BillingRepositoryErr::FailToConvertWithOtherData)?
            .ok_or(BillingRepositoryErr::FailToConvertWithOtherData)?
            .into();

        let all_retrieves_data_ids = BillingRetrieveDataDb::find()
            .filter(billing_retrieve_data_db::Column::BillingId.eq(model.billing_id))
            .all(conn)
            .await
            .map_err(|_| BillingRepositoryErr::FailToConvertWithOtherData)?
            .iter()
            .map(|billing| billing.retrieve_data_id)
            .collect::<Vec<i64>>();

        let retrieves_data_with_price = RetrieveDataDb::find()
            .filter(retrieve_data_db::Column::RetrieveDataId.is_in(all_retrieves_data_ids))
            .find_also_related(price::Entity)
            .order_by(retrieve_data_db::Column::Date, Order::Asc)
            .all(conn)
            .await
            .map_err(|_| BillingRepositoryErr::FailToConvertWithOtherData)?;

        let from = *retrieves_data_with_price
            .iter()
            .map(|(data, _)| data.date)
            .collect::<Vec<NaiveDateTime>>()
            .first()
            .ok_or(BillingRepositoryErr::FailToConvertWithOtherData)?;

        let to = *retrieves_data_with_price
            .iter()
            .map(|(data, _)| data.date)
            .collect::<Vec<NaiveDateTime>>()
            .last()
            .ok_or(BillingRepositoryErr::FailToConvertWithOtherData)?;

        let bill = retrieves_data_with_price
            .iter()
            .map(|(data, price)| data.amount as f64 * price.as_ref().unwrap().value as f64)
            .sum::<f64>();

        let amount = retrieves_data_with_price
            .iter()
            .map(|(data, _)| data.amount)
            .sum::<i64>();

        Ok(Billing {
            billing_id: model.billing_id,
            customer_id: model.customer_id,
            date: model.date,
            customer,
            from,
            to,
            bill,
            amount,
        })
    }
    pub async fn get_by_customer_id(
        &mut self,
        customer_id: i64,
    ) -> Result<Vec<Billing>, BillingRepositoryErr> {
        let conn = get_database_connection().await;

        let customer: Customer = customer::Entity::find_by_id(customer_id)
            .one(conn)
            .await
            .map_err(|_| BillingRepositoryErr::FailToGeyByCustomerId)?
            .ok_or(BillingRepositoryErr::FailToGeyByCustomerId)?
            .into();

        let billings_db = BillingDb::find()
            .filter(billing_db::Column::CustomerId.eq(customer_id))
            .find_with_related(BillingRetrieveDataDb)
            .all(conn)
            .await
            .map_err(|_| BillingRepositoryErr::FailToGeyByCustomerId)?;

        let mut billings = Vec::<Billing>::new();

        for billing in billings_db {
            let retrieve_data_ids = billing
                .1
                .iter()
                .map(|brd| brd.retrieve_data_id)
                .collect::<Vec<i64>>();

            let retrieve_data_with_price = RetrieveDataDb::find()
                .filter(retrieve_data_db::Column::CustomerId.eq(customer_id))
                .filter(retrieve_data_db::Column::RetrieveDataId.is_in(retrieve_data_ids))
                .order_by(retrieve_data_db::Column::Date, Order::Asc)
                .find_also_related(price::Entity)
                .all(conn)
                .await
                .map_err(|_| BillingRepositoryErr::FailToGeyByCustomerId)?;

            let retrieve_data = retrieve_data_with_price
                .iter()
                .map(|(data, _)| data.clone())
                .collect::<Vec<RetrieveDataModel>>();

            let from = retrieve_data
                .first()
                .ok_or(BillingRepositoryErr::FailToGeyByCustomerId)?
                .date;

            let to = retrieve_data
                .last()
                .ok_or(BillingRepositoryErr::FailToGeyByCustomerId)?
                .date;

            let amount = retrieve_data.iter().map(|data| data.amount).sum::<i64>();

            let bill = retrieve_data_with_price
                .iter()
                .map(|(data, price)| data.amount as f64 * price.as_ref().unwrap().value as f64)
                .sum::<f64>();

            let bill = Billing {
                billing_id: billing.0.billing_id,
                customer_id,
                date: billing.0.date,
                customer: customer.clone(),
                from,
                to,
                bill,
                amount,
            };
            billings.push(bill);
        }

        Ok(billings)
    }
}

impl BaseRepository<Billing> for BillingRepository {
    async fn create(&mut self, model: Billing) -> Result<i64, BaseRepositoryErr> {
        let active_model = model.to_active_without_id();
        let result = BillingDb.create(active_model).await;

        match result {
            Ok(created_model) => Ok(created_model.billing_id),
            Err(_) => Err(BaseRepositoryErr::FailToCreate),
        }
    }

    async fn read(&mut self, id: i64) -> Result<Option<Billing>, BaseRepositoryErr> {
        match BillingDb.get_by_id(id).await {
            Ok(model) => {
                let model = model.ok_or(BaseRepositoryErr::FailToRead)?;
                todo!()
                // Ok(Some(model.into()))
            }
            Err(_) => Err(BaseRepositoryErr::FailToCreate),
        }
    }

    async fn update(&mut self, model: Billing) -> Result<Billing, BaseRepositoryErr> {
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

    use super::*;
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
}
