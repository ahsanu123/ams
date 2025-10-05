#![allow(dead_code)]
use actix_web::{
    App, HttpResponse, Responder,
    dev::{ServiceFactory, ServiceRequest},
    post,
    web::Json,
};
use ams_entity::taking_record_table;
use chrono::NaiveDateTime;
use serde::Deserialize;

mod request_model {
    use super::*;

    #[derive(Deserialize)]
    pub struct AddNewTakingRecord {
        pub user_id: i32,
        pub amount: i32,
    }

    #[derive(Deserialize)]
    pub struct GetTakingRecordByUserId {
        pub user_id: i32,
    }

    #[derive(Deserialize)]
    pub struct UpdateTakingRecord {
        record: taking_record_table::Model,
    }

    #[derive(Deserialize)]
    pub struct GetTakingRecordByMonth {
        record: taking_record_table::Model,
    }

    #[derive(Deserialize)]
    pub struct GetTakingRecordByUserIdAndMonth {
        user_id: i32,
        date: NaiveDateTime,
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

#[post("/taking-record/add-new-taking-record")]
pub async fn add_new_taking_record(
    request: Json<request_model::AddNewTakingRecord>,
) -> impl Responder {
    HttpResponse::Ok()
}

#[post("/taking-record/get-taking-record-by-user-id")]
pub async fn get_taking_record_by_user_id(
    request: Json<request_model::GetTakingRecordByUserId>,
) -> impl Responder {
    HttpResponse::Ok()
}

#[post("/taking-record/upsert-taking-record")]
pub async fn upsert_taking_record(
    request: Json<request_model::UpdateTakingRecord>,
) -> impl Responder {
    HttpResponse::Ok()
}

#[post("/taking-record/get-taking-record-by-date")]
pub async fn get_taking_record_by_month(
    request: Json<request_model::GetTakingRecordByMonth>,
) -> impl Responder {
    HttpResponse::Ok()
}

#[post("/taking-record/get-taking-record-by-user-id-and-date")]
pub async fn get_taking_record_by_user_id_and_month(
    request: Json<request_model::GetTakingRecordByUserIdAndMonth>,
) -> impl Responder {
    HttpResponse::Ok()
}
