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
        billing_controller::{BillingControllerTrait, BillingGetAllProps},
    },
    models::billing::BillingCreate,
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
        self.service(get_all).service(create)
        // .service(get_latest_by_customer_id)
        //     .service(update)
        //     .service(delete)
    }
}

#[utoipa::path(
    get,
    tag = TAG_NAME,
    path = "/billing-info/get-all",
    responses(
        (status = 200, description = "success"),
        (status = NOT_FOUND, description = "not found")
    ),
    params(BillingGetAllProps),
    operation_id = "0095973a-7ad3-4111-a2f4-19ac7a91fabb", 
)]
#[get("/billing-info/get-all")]
pub async fn get_all(_passkey: PassKey, query: Query<BillingGetAllProps>) -> impl Responder {
    let result = BILLING_CONTROLLER.lock().await.get_all(query.0).await;

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
        (status = 200, description = "success"),
        (status = NOT_FOUND, description = "not found")
    ),
    request_body(
        content =  BillingCreate,
        content_type =  "application/json",
    ),
    operation_id = "c2f42b36-bddd-4276-a387-35d371d05745", 
)]
#[post("/billing/create")]
pub async fn create(_passkey: PassKey, request: Json<BillingCreate>) -> impl Responder {
    let result = BILLING_CONTROLLER.lock().await.create(request.0).await;

    match result {
        Ok(infos) => HttpResponse::Ok().json(infos),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}
