#![allow(dead_code)]
use actix_web::{
    App, HttpResponse, Responder,
    dev::{ServiceFactory, ServiceRequest},
    get, post,
    web::Json,
};
use ams_entity::user_table;
use serde::Deserialize;

mod request_model {
    use super::*;

    #[derive(Deserialize)]
    pub struct InsertNewUser {
        new_user: user_table::Model,
    }

    #[derive(Deserialize)]
    pub struct UpsertUser {
        user: user_table::Model,
    }
}

pub trait UserManagementServiceExtensionTrait {
    fn register_user_management_endpoints(self) -> Self;
}

impl<T> UserManagementServiceExtensionTrait for App<T>
where
    T: ServiceFactory<ServiceRequest, Config = (), Error = actix_web::Error, InitError = ()>,
{
    fn register_user_management_endpoints(self) -> Self {
        self.service(insert_new_user)
            .service(get_all_user)
            .service(get_all_active_user)
            .service(upsert_user)
    }
}

#[post("/user-management/insert-new-user")]
pub async fn insert_new_user(request: Json<request_model::InsertNewUser>) -> impl Responder {
    HttpResponse::Ok()
}

#[get("/user-management/get-all-user")]
pub async fn get_all_user() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/user-management/get-all-active-user")]
pub async fn get_all_active_user() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/user-management/upsert-user")]
pub async fn upsert_user(request: Json<request_model::UpsertUser>) -> impl Responder {
    HttpResponse::Ok()
}
