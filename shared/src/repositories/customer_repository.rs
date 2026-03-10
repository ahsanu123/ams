use crate::{
    models::{
        customer::{Customer, CustomerUpdate},
        to_active_model_trait::ToActiveModel,
    },
    repositories::{
        base_repository_trait::{BaseRepositoryErr, BaseRepositoryWithCRUType},
        database_connection::get_database_connection,
        generic_crud_repository::GenericCrudRepository,
    },
};

use ams_entity::customer as customer_db;
use ams_entity::prelude::Customer as CustomerDb;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

pub enum CustomerRepositoryErr {
    FailToGetAll,
    FailToGetAllActive,
}

pub struct CustomerRepository;

impl CustomerRepository {
    pub async fn check_if_customer_name_exist(
        &mut self,
        customer_name: String,
    ) -> Result<bool, CustomerRepositoryErr> {
        let conn = get_database_connection().await;

        let maybe_customer = CustomerDb::find()
            .filter(customer_db::Column::CustomerName.eq(customer_name))
            .one(conn)
            .await
            .map_err(|_| CustomerRepositoryErr::FailToGetAllActive)?;

        Ok(maybe_customer.is_some())
    }

    pub async fn get_is_active(
        &mut self,
        is_active: bool,
    ) -> Result<Vec<Customer>, CustomerRepositoryErr> {
        let conn = get_database_connection().await;

        let data = CustomerDb::find()
            .filter(customer_db::Column::IsActive.eq(is_active))
            .all(conn)
            .await
            .map_err(|_| CustomerRepositoryErr::FailToGetAllActive)?;

        let data = data
            .iter()
            .map(|customer| customer.into())
            .collect::<Vec<Customer>>();

        Ok(data)
    }

    pub async fn get_all(&mut self) -> Result<Vec<Customer>, CustomerRepositoryErr> {
        let conn = get_database_connection().await;

        let data = CustomerDb::find()
            .all(conn)
            .await
            .map_err(|_| CustomerRepositoryErr::FailToGetAll)?;

        let data = data
            .iter()
            .map(|customer| customer.into())
            .collect::<Vec<Customer>>();

        Ok(data)
    }

    pub async fn get_first_customer(&mut self) -> Result<Customer, CustomerRepositoryErr> {
        let conn = get_database_connection().await;

        let data = CustomerDb::find()
            .order_by_id_asc()
            .one(conn)
            .await
            .map_err(|_| CustomerRepositoryErr::FailToGetAll)?
            .ok_or(CustomerRepositoryErr::FailToGetAll)?;

        Ok(data.into())
    }
}

impl BaseRepositoryWithCRUType for CustomerRepository {
    type CreateType = Customer;
    type ReturnType = Customer;
    type UpdateType = CustomerUpdate;

    async fn create(&mut self, model: Customer) -> Result<i64, BaseRepositoryErr> {
        let active_model = model.to_active_without_id();
        let result = CustomerDb.create(active_model).await;

        match result {
            Ok(created_model) => Ok(created_model.customer_id),
            Err(_) => Err(BaseRepositoryErr::FailToCreate),
        }
    }

    async fn read(&mut self, id: i64) -> Result<Option<Customer>, BaseRepositoryErr> {
        match CustomerDb.get_by_id(id).await {
            Ok(model) => {
                let model = model.ok_or(BaseRepositoryErr::FailToRead)?;
                Ok(Some(model.into()))
            }
            Err(_) => Err(BaseRepositoryErr::FailToCreate),
        }
    }

    async fn update(&mut self, model: CustomerUpdate) -> Result<Customer, BaseRepositoryErr> {
        let active_model = model.to_active_with_id();
        let update_result = CustomerDb.update_by_model(active_model).await;

        match update_result {
            Ok(model) => Ok(model.into()),
            Err(_) => Err(BaseRepositoryErr::FailToUpdate),
        }
    }

    async fn delete(&mut self, id: i64) -> Result<u64, BaseRepositoryErr> {
        match CustomerDb.delete_by_model_id(id).await {
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
