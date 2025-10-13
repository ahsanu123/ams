#![allow(dead_code)]
use actix_web::{
    App, HttpResponse, Responder,
    dev::{ServiceFactory, ServiceRequest},
    get, post,
    web::Json,
};
use ams_entity::user_table;
use ams_shared::commands::user_management_command::{
    UserManagementCommand, UserManagementCommandTrait,
};
use serde::Deserialize;
use utoipa::ToSchema;

mod request_model {
    use super::*;

    #[derive(Deserialize, ToSchema)]
    #[serde(rename_all = "camelCase")]
    pub struct InsertNewUser {
        pub new_user: user_table::Model,
    }

    #[derive(Deserialize, ToSchema)]
    #[serde(rename_all = "camelCase")]
    pub struct UpsertUser {
        pub user: user_table::Model,
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

#[utoipa::path(
    post,
    path = "/user-management/insert-new-user",
    responses(
        (status = 200, description = "success"),
        (status = NOT_FOUND, description = "not found")
    ),
    request_body(
        content =  request_model::InsertNewUser,
        content_type =  "application/json",
    )
)]
#[post("/user-management/insert-new-user")]
pub async fn insert_new_user(request: Json<request_model::InsertNewUser>) -> impl Responder {
    let result = UserManagementCommand::insert_new_user(request.new_user.clone()).await;
    HttpResponse::Ok().json(result)
}

#[utoipa::path(
    get,
    path = "/user-management/get-all-user",
    responses(
        (status = 200, description = "success"),
        (status = NOT_FOUND, description = "not found")
    )
)]
#[get("/user-management/get-all-user")]
pub async fn get_all_user() -> impl Responder {
    let result = UserManagementCommand::get_all_user().await;
    HttpResponse::Ok().json(result)
}

#[utoipa::path(
    get,
    path = "/user-management/get-all-active-user",
    responses(
        (status = 200, description = "success"),
        (status = NOT_FOUND, description = "not found")
    )
)]
#[get("/user-management/get-all-active-user")]
pub async fn get_all_active_user() -> impl Responder {
    let result = UserManagementCommand::get_all_user().await;
    HttpResponse::Ok().json(result)
}

#[utoipa::path(
    post,
    path = "/user-management/upsert-user",
    responses(
        (status = 200, description = "success"),
        (status = NOT_FOUND, description = "not found")
    ),
    request_body(
        content =  request_model::UpsertUser,
        content_type =  "application/json",
    )
)]
#[post("/user-management/upsert-user")]
pub async fn upsert_user(request: Json<request_model::UpsertUser>) -> impl Responder {
    let result = UserManagementCommand::upsert_user(request.user.clone()).await;
    HttpResponse::Ok().json(result)
}
