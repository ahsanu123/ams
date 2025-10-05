#![allow(dead_code)]
use actix_web::{
    App, HttpResponse, Responder,
    dev::{ServiceFactory, ServiceRequest},
    post,
    web::Json,
};
use ams_shared::commands::customer_money_command::{
    CustomerMoneyCommand, CustomerMoneyCommandTrait,
};
use serde::Deserialize;
use utoipa::ToSchema;

mod request_model {
    use super::*;
    #[derive(Deserialize, ToSchema)]
    pub struct AddMoney {
        pub user_id: i64,
        pub amount: i64,
    }

    #[derive(Deserialize, ToSchema)]
    pub struct GetAllUserMoney {
        pub user_id: i64,
    }
}

pub trait CustomerServiceExtensionTrait {
    fn register_customer_endpoints(self) -> Self;
}

impl<T> CustomerServiceExtensionTrait for App<T>
where
    T: ServiceFactory<ServiceRequest, Config = (), Error = actix_web::Error, InitError = ()>,
{
    fn register_customer_endpoints(self) -> Self {
        self.service(add_money).service(get_all_user_money_history)
    }
}

#[utoipa::path(
    post,
    path = "/customer/add-money",
    responses(
        (status = 200, description = "success"),
        (status = NOT_FOUND, description = "not found")
    ),
    request_body(
        content =  request_model::AddMoney,
        content_type =  "application/json",
    )
)]
#[post("/customer/add-money")]
pub async fn add_money(request: Json<request_model::AddMoney>) -> impl Responder {
    let result = CustomerMoneyCommand::add_money(request.user_id, request.amount)
        .await
        .unwrap();
    HttpResponse::Ok().json(result)
}

#[utoipa::path(
    post,
    path = "/customer/get-all-user-money",
    responses(
        (status = 200, description = "success"),
        (status = NOT_FOUND, description = "not found")
    ),
    request_body(
        content =  request_model::GetAllUserMoney,
        content_type =  "application/json",
    )
)]
#[post("/customer/get-all-user-money")]
pub async fn get_all_user_money_history(
    request: Json<request_model::GetAllUserMoney>,
) -> impl Responder {
    let result = CustomerMoneyCommand::get_all_user_money_history(request.user_id).await;
    HttpResponse::Ok().json(result)
}
