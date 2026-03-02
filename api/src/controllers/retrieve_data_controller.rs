#![allow(dead_code)]
use actix_web::{
    App, HttpResponse, Responder, delete,
    dev::{ServiceFactory, ServiceRequest},
    get, post,
    web::{Json, Path, Query},
};

use ams_shared::{
    controllers::{
        RETRIEVE_DATA_CONTROLLER,
        retrieve_data_controller::{RetrieveDataControllerTrait, RetrieveDataGetAllProps},
    },
    models::retrieve_data::retrieve_data_create_or_update::{
        RetrieveDataCreate, RetrieveDataCreateOrUpdate, RetrieveDataCreateWithDate,
    },
};

static TAG_NAME: &str = "Retrieve Data Controller";

pub trait TakingRecordServiceExtensionTrait {
    fn register_retrieve_data_endpoints(self) -> Self;
}

impl<T> TakingRecordServiceExtensionTrait for App<T>
where
    T: ServiceFactory<ServiceRequest, Config = (), Error = actix_web::Error, InitError = ()>,
{
    fn register_retrieve_data_endpoints(self) -> Self {
        self.service(create)
            .service(create_with_date)
            .service(update)
            .service(get_all)
            .service(delete)
    }
}

#[utoipa::path(
    post,
    tag = TAG_NAME,
    path = "/retrieve-data/create",
    responses(
        (status = 200, description = "success"),
        (status = NOT_FOUND, description = "not found")
    ),
    request_body(
        content =  RetrieveDataCreate ,
        content_type =  "application/json",
    )
)]
#[post("/retrieve-data/create")]
pub async fn create(request: Json<RetrieveDataCreate>) -> impl Responder {
    let result = RETRIEVE_DATA_CONTROLLER
        .lock()
        .await
        .create(request.0)
        .await;

    match result {
        Ok(data) => HttpResponse::Created().json(data),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

#[utoipa::path(
    post,
    tag = TAG_NAME,
    path = "/retrieve-data/create-with-date",
    responses(
        (status = 200, description = "success"),
        (status = NOT_FOUND, description = "not found")
    ),
    request_body(
        content =  RetrieveDataCreateWithDate ,
        content_type =  "application/json",
    )
)]
#[post("/retrieve-data/create-with-date")]
pub async fn create_with_date(request: Json<RetrieveDataCreateWithDate>) -> impl Responder {
    let result = RETRIEVE_DATA_CONTROLLER
        .lock()
        .await
        .create_wd(request.0)
        .await;

    match result {
        Ok(data) => HttpResponse::Created().json(data),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

#[utoipa::path(
    post,
    tag = TAG_NAME,
    path = "/retrieve-data/update",
    responses(
        (status = 200, description = "success"),
        (status = NOT_FOUND, description = "not found")
    ),
    request_body(
        content =  RetrieveDataCreateOrUpdate,
        content_type =  "application/json",
    )
)]
#[post("/retrieve-data/update")]
pub async fn update(request: Json<RetrieveDataCreateOrUpdate>) -> impl Responder {
    let result = RETRIEVE_DATA_CONTROLLER
        .lock()
        .await
        .update(request.0)
        .await;

    match result {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

#[utoipa::path(
    get,
    tag = TAG_NAME,
    path = "/retrieve-data/get-all",
    responses(
        (status = 200, description = "success"),
        (status = NOT_FOUND, description = "not found")
    ),
    params(RetrieveDataGetAllProps)
)]
#[get("/retrieve-data/get-all")]
pub async fn get_all(request: Query<RetrieveDataGetAllProps>) -> impl Responder {
    let result = RETRIEVE_DATA_CONTROLLER
        .lock()
        .await
        .get_all(request.0)
        .await;

    match result {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}

#[utoipa::path(
    delete,
    tag = TAG_NAME,
    path = "/retrieve-data/delete/{retrieve_data_id}",
    params(
        ("retrieve_data_id" = i64, Path, description = "id to delete"),
    ),
    responses(
        (status = 200, description = "success"),
        (status = NOT_FOUND, description = "not found")
    ),
)]
#[delete("/retrieve-data/delete/{retrieve_data_id}")]
pub async fn delete(retrieve_data_id: Path<i64>) -> impl Responder {
    let result = RETRIEVE_DATA_CONTROLLER
        .lock()
        .await
        .delete(*retrieve_data_id)
        .await;

    match result {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(err) => HttpResponse::BadRequest().json(err),
    }
}
