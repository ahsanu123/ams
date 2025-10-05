#![allow(dead_code)]
use actix_web::{
    App, HttpResponse, Responder,
    dev::{ServiceFactory, ServiceRequest},
    post,
    web::Json,
};
use ams_entity::taking_record_table;
use ams_shared::commands::taking_record_command::{TakingRecordCommand, TakingRecordCommandTrait};
use chrono::NaiveDateTime;
use serde::Deserialize;
use utoipa::ToSchema;

mod request_model {
    use super::*;

    #[derive(Deserialize, ToSchema)]
    pub struct AddNewTakingRecord {
        pub user_id: i32,
        pub amount: i32,
    }

    #[derive(Deserialize, ToSchema)]
    pub struct GetTakingRecordByUserId {
        pub user_id: i32,
    }

    #[derive(Deserialize, ToSchema)]
    pub struct UpdateTakingRecord {
        pub record: taking_record_table::Model,
    }

    #[derive(Deserialize, ToSchema)]
    pub struct GetTakingRecordByMonth {
        pub date: NaiveDateTime,
    }

    #[derive(Deserialize, ToSchema)]
    pub struct GetTakingRecordByUserIdAndMonth {
        pub user_id: i32,
        pub date: NaiveDateTime,
    }
}

pub trait TakingRecordServiceExtensionTrait {
    fn register_taking_record_endpoints(self) -> Self;
}

impl<T> TakingRecordServiceExtensionTrait for App<T>
where
    T: ServiceFactory<ServiceRequest, Config = (), Error = actix_web::Error, InitError = ()>,
{
    fn register_taking_record_endpoints(self) -> Self {
        self.service(add_new_taking_record)
            .service(get_taking_record_by_user_id)
            .service(upsert_taking_record)
            .service(get_taking_record_by_month)
            .service(get_taking_record_by_user_id_and_month)
    }
}
#[utoipa::path(
    post,
    path = "/taking-record/add-new-taking-record",
    responses(
        (status = 200, description = "success"),
        (status = NOT_FOUND, description = "not found")
    ),
    request_body(
        content =  request_model::AddNewTakingRecord,
        content_type =  "application/json",
    )
)]
#[post("/taking-record/add-new-taking-record")]
pub async fn add_new_taking_record(
    request: Json<request_model::AddNewTakingRecord>,
) -> impl Responder {
    let result = TakingRecordCommand::add_new_taking_record(request.user_id, request.amount).await;
    HttpResponse::Ok().json(result)
}

#[utoipa::path(
    post,
    path = "/taking-record/get-taking-record-by-user-id",
    responses(
        (status = 200, description = "success"),
        (status = NOT_FOUND, description = "not found")
    ),
    request_body(
        content =  request_model::GetTakingRecordByUserId,
        content_type =  "application/json",
    )
)]
#[post("/taking-record/get-taking-record-by-user-id")]
pub async fn get_taking_record_by_user_id(
    request: Json<request_model::GetTakingRecordByUserId>,
) -> impl Responder {
    let result = TakingRecordCommand::get_taking_record_by_user_id(request.user_id).await;
    HttpResponse::Ok().json(result)
}

#[utoipa::path(
    post,
    path = "/taking-record/upsert-taking-record",
    responses(
        (status = 200, description = "success"),
        (status = NOT_FOUND, description = "not found")
    ),
    request_body(
        content =  request_model::UpdateTakingRecord,
        content_type =  "application/json",
    )
)]
#[post("/taking-record/upsert-taking-record")]
pub async fn upsert_taking_record(
    request: Json<request_model::UpdateTakingRecord>,
) -> impl Responder {
    let result = TakingRecordCommand::upsert_taking_record(request.record.clone()).await;
    HttpResponse::Ok().json(result)
}

#[utoipa::path(
    post,
    path = "/taking-record/get-taking-record-by-date",
    responses(
        (status = 200, description = "success"),
        (status = NOT_FOUND, description = "not found")
    ),
    request_body(
        content =  request_model::GetTakingRecordByMonth,
        content_type =  "application/json",
    )
)]
#[post("/taking-record/get-taking-record-by-date")]
pub async fn get_taking_record_by_month(
    request: Json<request_model::GetTakingRecordByMonth>,
) -> impl Responder {
    let result = TakingRecordCommand::get_taking_record_by_month(request.date).await;
    HttpResponse::Ok().json(result)
}

#[utoipa::path(
    post,
    path = "/taking-record/get-taking-record-by-user-id-and-date",
    responses(
        (status = 200, description = "success"),
        (status = NOT_FOUND, description = "not found")
    ),
    request_body(
        content =  request_model::GetTakingRecordByUserIdAndMonth,
        content_type =  "application/json",
    )
)]
#[post("/taking-record/get-taking-record-by-user-id-and-date")]
pub async fn get_taking_record_by_user_id_and_month(
    request: Json<request_model::GetTakingRecordByUserIdAndMonth>,
) -> impl Responder {
    let result =
        TakingRecordCommand::get_taking_record_by_user_id_and_month(request.user_id, request.date)
            .await;
    HttpResponse::Ok().json(result)
}
