#![allow(dead_code)]
use actix_web::{
    App, HttpResponse, Responder,
    dev::{ServiceFactory, ServiceRequest},
    post,
    web::Json,
};
use ams_entity::payment_history_table;
use chrono::NaiveDateTime;
use serde::Deserialize;
use utoipa::{OpenApi, ToSchema};

mod request_model {
    use super::*;

    #[derive(Deserialize, ToSchema)]
    pub struct UpdateDregPrice {
        user_id: i32,
    }

    #[derive(Deserialize, ToSchema)]
    pub struct GetMonthSummaryByDate {
        date: NaiveDateTime,
    }

    #[derive(Deserialize, ToSchema)]
    pub struct GetPaymentRecord {
        user_id: i32,
        date: NaiveDateTime,
    }

    #[derive(Deserialize, ToSchema)]
    pub struct GetPaymentRecordByUserId {
        user_id: i32,
        date: NaiveDateTime,
    }

    #[derive(Deserialize, ToSchema)]
    pub struct UpdatePaymentRecord {
        record: payment_history_table::Model,
    }

    #[derive(Deserialize, ToSchema)]
    pub struct UpdateBulkPaymentRecord {
        records: Vec<payment_history_table::Model>,
        paid: bool,
    }
}

pub trait PaymentServiceExtensionTrait {
    fn register_payment_endpoints(self) -> Self;
}

impl<T> PaymentServiceExtensionTrait for App<T>
where
    T: ServiceFactory<ServiceRequest, Config = (), Error = actix_web::Error, InitError = ()>,
{
    fn register_payment_endpoints(self) -> Self {
        self.service(get_payment_record_by_user_id)
            .service(get_month_summary)
            .service(get_payment_record_by_user_id_and_month)
            .service(get_month_summary_by_user_id)
            .service(update_payment_record)
            .service(update_bulk_payment_record)
    }
}

#[utoipa::path(
    post,
    path = "/payment/get-payment-record",
    responses(
        (status = 200, description = "success"),
        (status = NOT_FOUND, description = "not found")
    )
)]
#[post("/payment/get-payment-record")]
pub async fn get_payment_record_by_user_id(
    request: Json<request_model::UpdateDregPrice>,
) -> impl Responder {
    HttpResponse::Ok()
}

#[utoipa::path(
    post,
    path = "/payment/get-month-summary",
    responses(
        (status = 200, description = "success"),
        (status = NOT_FOUND, description = "not found")
    )
)]
#[post("/payment/get-month-summary")]
pub async fn get_month_summary(
    request: Json<request_model::GetMonthSummaryByDate>,
) -> impl Responder {
    HttpResponse::Ok()
}

#[utoipa::path(
    post,
    path = "/payment/get-payment-record",
    responses(
        (status = 200, description = "success"),
        (status = NOT_FOUND, description = "not found")
    )
)]
#[post("/payment/get-payment-record")]
pub async fn get_payment_record_by_user_id_and_month(
    request: Json<request_model::GetPaymentRecord>,
) -> impl Responder {
    HttpResponse::Ok()
}

#[utoipa::path(
    post,
    path = "/payment/get-month-summary-by-user-id",
    responses(
        (status = 200, description = "success"),
        (status = NOT_FOUND, description = "not found")
    )
)]
#[post("/payment/get-month-summary-by-user-id")]
pub async fn get_month_summary_by_user_id(
    request: Json<request_model::GetPaymentRecordByUserId>,
) -> impl Responder {
    HttpResponse::Ok()
}

#[utoipa::path(
    post,
    path = "/payment/update-payment-record",
    responses(
        (status = 200, description = "success"),
        (status = NOT_FOUND, description = "not found")
    )
)]
#[post("/payment/update-payment-record")]
pub async fn update_payment_record(
    request: Json<request_model::UpdatePaymentRecord>,
) -> impl Responder {
    HttpResponse::Ok()
}

#[utoipa::path(
    post,
    path = "/payment/update-bulk-payment-record",
    responses(
        (status = 200, description = "success"),
        (status = NOT_FOUND, description = "not found")
    )
)]
#[post("/payment/update-bulk-payment-record")]
pub async fn update_bulk_payment_record(
    request: Json<request_model::UpdateBulkPaymentRecord>,
) -> impl Responder {
    HttpResponse::Ok()
}
