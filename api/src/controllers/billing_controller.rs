#![allow(dead_code)]
use crate::extractors::calculated_passkey_extractor::PassKey;
use actix_web::{
    App, HttpResponse, Responder, delete,
    dev::{ServiceFactory, ServiceRequest},
    get, post,
    web::{Json, Path, Query},
};
use ams_shared::{
    controllers::{
        BILLING_CONTROLLER,
        billing_controller::{BillingControllerTrait, BillingGetByProps, BillingInfoGetAllProps},
    },
    models::billing::{
        BillingCreate,
        billing_info::{BillingInfo, BillingInfoWithBalance},
    },
};

static TAG_NAME: &str = "Billing Controller";

pub trait BillingServiceExtensionTrait {
    fn register_billing_controller(self) -> Self;
}

impl<T> BillingServiceExtensionTrait for App<T>
where
    T: ServiceFactory<ServiceRequest, Config = (), Error = actix_web::Error, InitError = ()>,
{
    fn register_billing_controller(self) -> Self {
        self.service(get_all_billing_info)
            .service(create)
            .service(get_by)
            .service(get_all_billing)
    }
}

#[utoipa::path(
    get,
    tag = TAG_NAME,
    path = "/billing-info/get-all",
    responses(
        (status = 200, description = "success", body = Vec<BillingInfo>),
        (status = NOT_FOUND, description = "not found")
    ),
    params(BillingInfoGetAllProps),
    operation_id = "getAllBillingInfo", 
)]
#[get("/billing-info/get-all")]
pub async fn get_all_billing_info(
    _passkey: PassKey,
    query: Query<BillingInfoGetAllProps>,
) -> impl Responder {
    let result = BILLING_CONTROLLER
        .lock()
        .await
        .get_all_billing_info(query.0)
        .await;

    match result {
        Ok(infos) => HttpResponse::Ok().json(infos),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

#[utoipa::path(
    post,
    tag = TAG_NAME,
    path = "/billing/create",
    responses(
        (status = 200, description = "success", body = BillingInfo),
        (status = NOT_FOUND, description = "not found")
    ),
    request_body(
        content =  BillingCreate,
        content_type =  "application/json",
    ),
    operation_id = "postCreateBilling", 
)]
#[post("/billing/create")]
pub async fn create(_passkey: PassKey, request: Json<BillingCreate>) -> impl Responder {
    let result = BILLING_CONTROLLER.lock().await.create(request.0).await;

    match result {
        Ok(infos) => HttpResponse::Ok().json(infos),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

#[utoipa::path(
    get,
    tag = TAG_NAME,
    path = "/billing/get_by",
    responses(
        (status = 200, description = "success", body = Vec<BillingInfoWithBalance>),
        (status = NOT_FOUND, description = "not found")
    ),
    params(BillingGetByProps),
    operation_id = "getBillingByProps", 
)]
#[get("/billing/get_by")]
pub async fn get_by(_passkey: PassKey, query: Query<BillingGetByProps>) -> impl Responder {
    let result = BILLING_CONTROLLER.lock().await.get_by(query.0).await;

    match result {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

#[utoipa::path(
    get,
    tag = TAG_NAME,
    path = "/billing/get-all",
    responses(
        (status = 200, description = "success", body = Vec<BillingInfoWithBalance>),
        (status = NOT_FOUND, description = "not found")
    ),
    operation_id = "getAllBilling", 
)]
#[get("/billing/get-all")]
pub async fn get_all_billing(_passkey: PassKey) -> impl Responder {
    let result = BILLING_CONTROLLER.lock().await.get_all_billing().await;

    match result {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}
