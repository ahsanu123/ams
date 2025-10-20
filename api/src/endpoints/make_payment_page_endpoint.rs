#![allow(dead_code)]
use actix_web::{
    App, HttpResponse, Responder,
    dev::{ServiceFactory, ServiceRequest},
    get, post,
    web::Json,
};
use ams_shared::commands::make_payment_command::{MakePaymentCommand, MakePaymentCommandTrait};
use chrono::NaiveDateTime;
use serde::Deserialize;
use utoipa::ToSchema;

mod request_model {

    use super::*;
    #[derive(Deserialize, ToSchema)]
    #[serde(rename_all = "camelCase")]
    pub struct UserIdAndDateRequestModel {
        pub user_id: i32,
        pub date: NaiveDateTime,
    }
}

pub trait MakePaymentPageServiceExtensionTrait {
    fn register_make_payment_page_endpoints(self) -> Self;
}

impl<T> MakePaymentPageServiceExtensionTrait for App<T>
where
    T: ServiceFactory<ServiceRequest, Config = (), Error = actix_web::Error, InitError = ()>,
{
    fn register_make_payment_page_endpoints(self) -> Self {
        self.service(payment_page_get_page_model)
            .service(payment_page_make_payment)
    }
}

#[utoipa::path(
    post,
    path = "/make-payment-page/get-page-model",
    responses(
        (status = 200, description = "success"),
        (status = NOT_FOUND, description = "not found")
    ),
    request_body(
        content =  request_model::UserIdAndDateRequestModel,
        content_type =  "application/json",
    )
)]
#[post("/make-payment-page/get-page-model")]
pub async fn payment_page_get_page_model(
    request: Json<request_model::UserIdAndDateRequestModel>,
) -> impl Responder {
    let result = MakePaymentCommand::get_page_model(request.user_id, request.date)
        .await
        .unwrap();

    HttpResponse::Ok().json(result)
}

#[utoipa::path(
    post,
    path = "/make-payment-page/make-payment",
    responses(
        (status = 200, description = "success"),
        (status = NOT_FOUND, description = "not found")
    ),
    request_body(
        content =  request_model::UserIdAndDateRequestModel,
        content_type =  "application/json",
    )
)]
#[post("/make-payment-page/make-payment")]
pub async fn payment_page_make_payment(
    request: Json<request_model::UserIdAndDateRequestModel>,
) -> impl Responder {
    let result = MakePaymentCommand::make_payment(request.user_id, request.date)
        .await
        .unwrap();

    HttpResponse::Ok().json(result)
}
