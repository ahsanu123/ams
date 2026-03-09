#![allow(dead_code)]
use actix_web::{
    App, HttpResponse, Responder, delete,
    dev::{ServiceFactory, ServiceRequest},
    get, post,
    web::Path,
};
use ams_shared::{
    controllers::{PRICE_CONTROLLER, price_controller::PriceControllerTrait},
    models::price::Price,
};

use crate::extractors::calculated_passkey_extractor::PassKey;

static TAG_NAME: &str = "Price Controller";

pub trait PriceControllerServiceExtensionTrait {
    fn register_price_controller(self) -> Self;
}

impl<T> PriceControllerServiceExtensionTrait for App<T>
where
    T: ServiceFactory<ServiceRequest, Config = (), Error = actix_web::Error, InitError = ()>,
{
    fn register_price_controller(self) -> Self {
        self.service(get_latest)
            .service(update)
            .service(get_all)
            .service(delete)
    }
}

#[utoipa::path(
    get,
    tag = TAG_NAME,
    path = "/price/get-latest",
    responses(
        (status = 200, description = "success", body = Price),
        (status = NOT_FOUND, description = "not found")
    ),
    operation_id = "getLatestPrice", 
)]
#[get("/price/get-latest")]
pub async fn get_latest(_passkey: PassKey) -> impl Responder {
    let result = PRICE_CONTROLLER.lock().await.get_latest_dreg_price().await;

    match result {
        Ok(price) => HttpResponse::Ok().json(price),
        Err(err) => HttpResponse::NotFound().json(err),
    }
}

#[utoipa::path(
    post,
    tag = TAG_NAME,
    path = "/price/update/{new_price}",
    responses(
        (status = 200, description = "success", body = i64),
        (status = NOT_FOUND, description = "not found")
    ),
    params(
        ("new_price" = f32, Path, description = "new price")
    ),
    operation_id = "updatePrice", 
)]
#[post("/price/update/{new_price}")]
pub async fn update(_passkey: PassKey, new_price: Path<f32>) -> impl Responder {
    let result = PRICE_CONTROLLER
        .lock()
        .await
        .update_dreg_price(*new_price)
        .await;

    match result {
        Ok(price_id) => HttpResponse::Ok().json(price_id),
        Err(err) => HttpResponse::NotFound().json(err),
    }
}

#[utoipa::path(
    get,
    tag = TAG_NAME,
    path = "/price/get-all",
    responses(
        (status = 200, description = "success", body = Vec<Price>),
        (status = NOT_FOUND, description = "not found")
    ),
    operation_id = "getAllPrice", 
)]
#[get("/price/get-all")]
pub async fn get_all(_passkey: PassKey) -> impl Responder {
    let result = PRICE_CONTROLLER.lock().await.get_all_dreg_price().await;

    match result {
        Ok(prices) => HttpResponse::Ok().json(prices),
        Err(err) => HttpResponse::NotFound().json(err),
    }
}

#[utoipa::path(
    delete,
    tag = TAG_NAME,
    path = "/price/{price_id}",
    responses(
        (status = 200, description = "success", body = u64),
        (status = NOT_FOUND, description = "not found")
    ),
    params(
        ("price_id" = i64, Path, description = "price id to be deleted")
    ),
    operation_id = "deletePriceById", 
)]
#[delete("/price/{price_id}")]
pub async fn delete(_passkey: PassKey, price_id: Path<i64>) -> impl Responder {
    let result = PRICE_CONTROLLER.lock().await.delete_by_id(*price_id).await;

    match result {
        Ok(deleted_count) => HttpResponse::Ok().json(deleted_count),
        Err(err) => HttpResponse::NotFound().json(err),
    }
}
