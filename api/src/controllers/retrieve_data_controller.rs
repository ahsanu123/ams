#![allow(dead_code)]
use actix_web::{
    App, HttpResponse, Responder, delete,
    dev::{ServiceFactory, ServiceRequest},
    post,
    web::Json,
};

use ams_shared::{
    controllers::{
        RETRIEVE_DATA_CONTROLLER, retrieve_data_controller::RetrieveDataControllerTrait,
    },
    models::retrieve_data::retrieve_data_create_or_update::{
        RetrieveDataCreate, RetrieveDataCreateWithDate,
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
        self.service(create).service(create_with_date)
        // .service(get_taking_record_by_user_id)
        // .service(upsert_taking_record)
        // .service(get_taking_record_by_month)
        // .service(get_taking_record_by_user_id_and_month)
        // .service(get_taking_record_by_user_id_and_year)
        // .service(get_taking_record_by_day)
        // .service(upsert_taking_record_by_date)
        // .service(delete_taking_record_by_id)
        // .service(get_taking_record_by_user_id_and_month_range)
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
