use crate::{
    models::{
        balance::BalanceCreateOrUpdate,
        customer::{Customer, CustomerUpdate},
    },
    repositories::{BALANCE_REPO, CUSTOMER_REPO, base_repository_trait::BaseRepositoryWithCRUType},
};
use chrono::Local;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::{IntoParams, ToSchema};

#[derive(Serialize)]
pub enum CustomerControllerErr {
    CustomerNameAlreadyExist,
    FailToCreateNewCustomer,
    FailtToUpdate,
    FailToGetById,
    CantGetAll,
    FailToDelete,
    CustomerNotFound,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams, Clone, TS)]
#[into_params(parameter_in = Query)]
#[ts(export)]
pub struct CustomerGetAllProp {
    is_active: Option<bool>,
}

pub trait CustomerControllerTrait {
    fn create_new_customer(
        &mut self,
        customer_name: String,
    ) -> impl Future<Output = Result<i64, CustomerControllerErr>>;

    fn get_by_customer_id(
        &mut self,
        customer_id: i64,
    ) -> impl Future<Output = Result<Option<Customer>, CustomerControllerErr>>;

    fn get_all(
        &mut self,
        props: CustomerGetAllProp,
    ) -> impl Future<Output = Result<Vec<Customer>, CustomerControllerErr>>;

    fn get_first_customer(
        &mut self,
    ) -> impl Future<Output = Result<Customer, CustomerControllerErr>>;

    fn update(
        &mut self,
        customer: CustomerUpdate,
    ) -> impl Future<Output = Result<i64, CustomerControllerErr>>;

    fn delete(
        &mut self,
        customer_id: i64,
    ) -> impl Future<Output = Result<u64, CustomerControllerErr>>;
}

pub struct CustomerController;

impl CustomerControllerTrait for CustomerController {
    async fn create_new_customer(
        &mut self,
        customer_name: String,
    ) -> Result<i64, CustomerControllerErr> {
        let customer = Customer {
            customer_id: 0,
            customer_name: customer_name.clone(),
            is_active: true,
            is_admin: false,
            created_date: Local::now().naive_local(),
            updated_date: Local::now().naive_local(),
        };

        let is_customer_name_already_exist = CUSTOMER_REPO
            .lock()
            .await
            .check_if_customer_name_exist(customer_name)
            .await
            .map_err(|_| CustomerControllerErr::CustomerNameAlreadyExist)?;

        if is_customer_name_already_exist {
            return Err(CustomerControllerErr::CustomerNameAlreadyExist);
        }

        let customer_id = CUSTOMER_REPO
            .lock()
            .await
            .create(customer)
            .await
            .map_err(|_| CustomerControllerErr::FailToCreateNewCustomer)?;

        let balance_id = BALANCE_REPO
            .lock()
            .await
            .create(BalanceCreateOrUpdate::empty_balance(customer_id))
            .await
            .map_err(|_| CustomerControllerErr::FailToCreateNewCustomer)?;

        match balance_id {
            0 => Err(CustomerControllerErr::FailToCreateNewCustomer),
            customer_id => Ok(customer_id),
        }
    }

    async fn get_all(
        &mut self,
        props: CustomerGetAllProp,
    ) -> Result<Vec<Customer>, CustomerControllerErr> {
        match props {
            CustomerGetAllProp {
                is_active: Some(is_active),
            } => CUSTOMER_REPO
                .lock()
                .await
                .get_is_active(is_active)
                .await
                .map_err(|_| CustomerControllerErr::CantGetAll),

            CustomerGetAllProp { is_active: None } => CUSTOMER_REPO
                .lock()
                .await
                .get_all()
                .await
                .map_err(|_| CustomerControllerErr::CantGetAll),
        }
    }

    async fn update(&mut self, customer: CustomerUpdate) -> Result<i64, CustomerControllerErr> {
        let result = CUSTOMER_REPO
            .lock()
            .await
            .update(customer)
            .await
            .map_err(|_| CustomerControllerErr::FailtToUpdate)?;

        Ok(result.customer_id)
    }

    async fn get_by_customer_id(
        &mut self,
        customer_id: i64,
    ) -> Result<Option<Customer>, CustomerControllerErr> {
        let result = CUSTOMER_REPO
            .lock()
            .await
            .read(customer_id)
            .await
            .map_err(|_| CustomerControllerErr::CustomerNotFound)?;

        Ok(result)
    }

    async fn delete(&mut self, customer_id: i64) -> Result<u64, CustomerControllerErr> {
        let result = CUSTOMER_REPO
            .lock()
            .await
            .delete(customer_id)
            .await
            .map_err(|_| CustomerControllerErr::CustomerNotFound)?;

        Ok(result)
    }

    async fn get_first_customer(&mut self) -> Result<Customer, CustomerControllerErr> {
        let result = CUSTOMER_REPO
            .lock()
            .await
            .get_first_customer()
            .await
            .map_err(|_| CustomerControllerErr::CustomerNotFound)?;

        Ok(result)
    }
}
