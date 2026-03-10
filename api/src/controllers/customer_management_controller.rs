#![allow(dead_code)]
use std::ops::Deref;

use actix_web::{
    App, HttpResponse, Responder, delete,
    dev::{ServiceFactory, ServiceRequest},
    get, post,
    web::{Json, Path, Query},
};
use ams_entity::customer::Entity;
use ams_shared::{
    controllers::{
        CUSTOMER_CONTROLLER,
        customer_controller::{CustomerControllerTrait, CustomerGetAllProp},
    },
    models::customer::{Customer, CustomerUpdate},
};

use crate::extractors::calculated_passkey_extractor::PassKey;

static TAG_NAME: &str = "Customer Management Controller";

pub trait CustomerManagementServiceExtensionTrait {
    fn register_customer_controller(self) -> Self;
}

impl<T> CustomerManagementServiceExtensionTrait for App<T>
where
    T: ServiceFactory<ServiceRequest, Config = (), Error = actix_web::Error, InitError = ()>,
{
    fn register_customer_controller(self) -> Self {
        self.service(create)
            .service(update)
            .service(get_all)
            .service(get_first_customer)
            .service(get_by_id)
            .service(delete)
    }
}

#[utoipa::path(
    post,
    tag = TAG_NAME,
    path = "/customer/create/{customer_name}",
    responses(
        (status = 200, description = "success", body = i64),
        (status = NOT_FOUND, description = "not found")
    ),
    params(
        ("customer_name" = String, Path, description = "new customer name"),
    ),
    operation_id = "postCreateCustomerByCustomerName"
)]
#[post("/customer/create/{customer_name}")]
pub async fn create(_passkey: PassKey, customer_name: Path<String>) -> impl Responder {
    let result = CUSTOMER_CONTROLLER
        .lock()
        .await
        .create_new_customer(customer_name.deref().clone())
        .await;

    match result {
        Ok(id) => HttpResponse::Ok().json(id),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

#[utoipa::path(
    post,
    tag = TAG_NAME,
    path = "/customer/update",
    responses(
        (status = 200, description = "success", body = i64),
        (status = NOT_FOUND, description = "not found")
    ),
    request_body(
        content =  CustomerUpdate,
        content_type =  "application/json",
    ),
    operation_id = "postUpdateCustomer"
)]
#[post("/customer/update")]
pub async fn update(_passkey: PassKey, request: Json<CustomerUpdate>) -> impl Responder {
    let result = CUSTOMER_CONTROLLER.lock().await.update(request.0).await;

    match result {
        Ok(id) => HttpResponse::Ok().json(id),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

#[utoipa::path(
    get,
    tag = TAG_NAME,
    path = "/customer/first",
    responses(
        (status = 200, description = "success", body = Customer),
        (status = NOT_FOUND, description = "not found")
    ),
    operation_id = "getFirstCustomer", 
)]
#[get("/customer/first")]
pub async fn get_first_customer(_passkey: PassKey) -> impl Responder {
    let result = CUSTOMER_CONTROLLER.lock().await.get_first_customer().await;

    match result {
        Ok(customer) => HttpResponse::Ok().json(customer),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

#[utoipa::path(
    get,
    tag = TAG_NAME,
    path = "/customer/get_all",
    responses(
        (status = 200, description = "success", body = Vec<Customer>),
        (status = NOT_FOUND, description = "not found")
    ),
    params(CustomerGetAllProp),
    operation_id = "getAllCustomerByProps", 
)]
#[get("/customer/get_all")]
pub async fn get_all(_passkey: PassKey, query: Query<CustomerGetAllProp>) -> impl Responder {
    let result = CUSTOMER_CONTROLLER.lock().await.get_all(query.0).await;

    match result {
        Ok(customers) => HttpResponse::Ok().json(customers),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

#[utoipa::path(
    get,
    tag = TAG_NAME,
    path = "/customer/{customer_id}",
    responses(
        (status = 200, description = "success", body = Option<Customer>),
        (status = NOT_FOUND, description = "not found")
    ),
    params(
        ("customer_id" = i64, Path, description = "customer id to search")
    ),
    operation_id = "getCustomerByCustomerId"
)]
#[get("/customer/{customer_id}")]
pub async fn get_by_id(_passkey: PassKey, customer_id: Path<i64>) -> impl Responder {
    let result = CUSTOMER_CONTROLLER
        .lock()
        .await
        .get_by_customer_id(*customer_id)
        .await;

    match result {
        Ok(customer) => HttpResponse::Ok().json(customer),
        Err(err) => HttpResponse::NotFound().json(err),
    }
}

#[utoipa::path(
    delete,
    tag = TAG_NAME,
    path = "/customer/{customer_id}",
    responses(
        (status = 200, description = "success", body = u64),
        (status = NOT_FOUND, description = "not found")
    ),
    params(
        ("customer_id" = i64, Path, description = "customer id to delete")
    ),
    operation_id = "deleteCustomerByCustomerId"
)]
#[delete("/customer/{customer_id}")]
pub async fn delete(_passkey: PassKey, customer_id: Path<i64>) -> impl Responder {
    let result = CUSTOMER_CONTROLLER.lock().await.delete(*customer_id).await;

    match result {
        Ok(deleted_count) => HttpResponse::Ok().json(deleted_count),
        Err(err) => HttpResponse::NotFound().json(err),
    }
}
