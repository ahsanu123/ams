#![allow(dead_code)]
use actix_web::{
    App, HttpResponse, Responder, delete,
    dev::{ServiceFactory, ServiceRequest},
    get, post,
    web::{Json, Path, Query},
};

use crate::extractors::calculated_passkey_extractor::PassKey;
use ams_shared::{
    controllers::{
        BALANCE_CONTROLLER, RETRIEVE_DATA_CONTROLLER,
        balance_controller::{BalanceControllerTrait, BalanceGetAllProps},
        retrieve_data_controller::{RetrieveDataControllerTrait, RetrieveDataGetAllProps},
    },
    models::{
        balance::{BalanceCreateOrUpdate, BalanceCreateOrUpdateWithoutChangedValue},
        retrieve_data::retrieve_data_create_or_update::{
            RetrieveDataCreate, RetrieveDataCreateOrUpdate, RetrieveDataCreateWithDate,
        },
    },
};

static TAG_NAME: &str = "Balance Controller";

pub trait BalanceServiceExtensionTrait {
    fn register_balance_controller(self) -> Self;
}

impl<T> BalanceServiceExtensionTrait for App<T>
where
    T: ServiceFactory<ServiceRequest, Config = (), Error = actix_web::Error, InitError = ()>,
{
    fn register_balance_controller(self) -> Self {
        self.service(get_all)
            .service(get_latest_by_customer_id)
            .service(add_balance)
        //     .service(update)
        //     .service(delete)
    }
}

#[utoipa::path(
    post,
    tag = TAG_NAME,
    path = "/balance",
    responses(
        (status = 200, description = "success"),
        (status = NOT_FOUND, description = "not found")
    ),
    request_body(
        content =  BalanceCreateOrUpdateWithoutChangedValue,
        content_type =  "application/json",
    )
)]
#[post("/balance")]
pub async fn add_balance(
    _passkey: PassKey,
    request: Json<BalanceCreateOrUpdateWithoutChangedValue>,
) -> impl Responder {
    let result = BALANCE_CONTROLLER.lock().await.add_balance(request.0).await;

    match result {
        Ok(id) => HttpResponse::Ok().json(id),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

#[utoipa::path(
    get,
    tag = TAG_NAME,
    path = "/balance/latest/{customer_id}",
    responses(
        (status = 200, description = "success"),
        (status = NOT_FOUND, description = "not found")
    ),
    params(
        ("customer_id" = i64, Path, description = "customer id to get info"),
    ),
)]
#[get("/balance/latest/{customer_id}")]
pub async fn get_latest_by_customer_id(
    _passkey: PassKey,
    customer_id: Path<i64>,
) -> impl Responder {
    let result = BALANCE_CONTROLLER
        .lock()
        .await
        .get_latest(*customer_id)
        .await;

    match result {
        Ok(customer) => HttpResponse::Ok().json(customer),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

#[utoipa::path(
    get,
    tag = TAG_NAME,
    path = "/balance/get_all",
    responses(
        (status = 200, description = "success"),
        (status = NOT_FOUND, description = "not found")
    ),
    params(BalanceGetAllProps)
)]
#[get("/balance/get_all")]
pub async fn get_all(_passkey: PassKey, query: Query<BalanceGetAllProps>) -> impl Responder {
    let result = BALANCE_CONTROLLER.lock().await.get_all(query.0).await;

    match result {
        Ok(balances) => HttpResponse::Ok().json(balances),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}
