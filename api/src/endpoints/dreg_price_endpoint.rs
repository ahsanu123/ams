#![allow(dead_code)]
use actix_web::{
    App, HttpResponse, Responder,
    dev::{ServiceFactory, ServiceRequest},
    get, post,
    web::Json,
};
use serde::Deserialize;
use utoipa::ToSchema;

mod request_model {
    use super::*;
    #[derive(Deserialize, ToSchema)]
    pub struct UpdateDregPrice {
        new_price: i64,
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
    )
)]
#[get("/dreg-price/get-latest-dreg-price")]
pub async fn get_latest_dreg_price() -> impl Responder {
    HttpResponse::Ok()
}

#[utoipa::path(
    post,
    path = "/dreg-price/update-dreg-price",
    responses(
        (status = 200, description = "success"),
        (status = NOT_FOUND, description = "not found")
    )
)]
#[post("/dreg-price/update-dreg-price")]
pub async fn update_dreg_price(request: Json<request_model::UpdateDregPrice>) -> impl Responder {
    HttpResponse::Ok()
}

#[utoipa::path(
    post,
    path = "/dreg-price/get-all-price",
    responses(
        (status = 200, description = "success"),
        (status = NOT_FOUND, description = "not found")
    )
)]
#[get("/dreg-price/get-all-price")]
pub async fn get_all_dreg_price() -> impl Responder {
    HttpResponse::Ok()
}
