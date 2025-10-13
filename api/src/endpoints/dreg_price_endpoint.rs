#![allow(dead_code)]
use actix_web::{
    App, HttpResponse, Responder,
    dev::{ServiceFactory, ServiceRequest},
    get, post,
    web::Json,
};
use ams_shared::commands::dreg_price_command::{DregPriceCommand, DregPriceCommandTrait};
use serde::Deserialize;
use utoipa::ToSchema;

mod request_model {
    use super::*;
    #[derive(Deserialize, ToSchema)]
    #[serde(rename_all = "camelCase")]
    pub struct UpdateDregPrice {
        pub new_price: i64,
    }
}

pub trait DregsPriceServiceExtensionTrait {
    fn register_dregs_price_endpoints(self) -> Self;
}

impl<T> DregsPriceServiceExtensionTrait for App<T>
where
    T: ServiceFactory<ServiceRequest, Config = (), Error = actix_web::Error, InitError = ()>,
{
    fn register_dregs_price_endpoints(self) -> Self {
        self.service(get_latest_dreg_price)
            .service(update_dreg_price)
            .service(get_all_dreg_price)
    }
}

#[utoipa::path(
    get,
    path = "/dreg-price/get-latest-dreg-price",
    responses(
        (status = 200, description = "success"),
        (status = NOT_FOUND, description = "not found")
    ),
)]
#[get("/dreg-price/get-latest-dreg-price")]
pub async fn get_latest_dreg_price() -> impl Responder {
    let result = DregPriceCommand::get_latest_dreg_price().await.unwrap();
    HttpResponse::Ok().json(result)
}

#[utoipa::path(
    post,
    path = "/dreg-price/update-dreg-price",
    responses(
        (status = 200, description = "success"),
        (status = NOT_FOUND, description = "not found")
    ),
    request_body(
        content =  request_model::UpdateDregPrice,
        content_type =  "application/json",
    )
)]
#[post("/dreg-price/update-dreg-price")]
pub async fn update_dreg_price(request: Json<request_model::UpdateDregPrice>) -> impl Responder {
    let result = DregPriceCommand::update_dreg_price(request.new_price).await;
    HttpResponse::Ok().json(result)
}

#[utoipa::path(
    get,
    path = "/dreg-price/get-all-price",
    responses(
        (status = 200, description = "success"),
        (status = NOT_FOUND, description = "not found")
    )
)]
#[get("/dreg-price/get-all-price")]
pub async fn get_all_dreg_price() -> impl Responder {
    let result = DregPriceCommand::get_all_dreg_price().await;
    HttpResponse::Ok().json(result)
}
