#![allow(dead_code)]
use actix_web::{
    App, HttpResponse, Responder,
    dev::{ServiceFactory, ServiceRequest},
    get, post,
    web::Json,
};
use ams_entity::user_table;
use ams_shared::{prelude::*, singletons::USER_MANAGEMENT_COMMAND};
use serde::Deserialize;
use utoipa::ToSchema;

static TAG_NAME: &str = "User Management Endpoint";

mod request_model {
    use super::*;

    #[derive(Deserialize, ToSchema)]
    #[serde(rename_all = "camelCase")]
    pub struct CreateNewUser {
        pub username: String,
    }

    #[derive(Deserialize, ToSchema)]
    #[serde(rename_all = "camelCase")]
    pub struct InsertNewUser {
        #[schema(inline)]
        pub new_user: user_table::Model,
    }

    #[derive(Deserialize, ToSchema)]
    #[serde(rename_all = "camelCase")]
    pub struct UpsertUser {
        #[schema(inline)]
        pub user: user_table::Model,
    }

    #[derive(Deserialize, ToSchema)]
    #[serde(rename_all = "camelCase")]
    pub struct UserIdRequest {
        pub user_id: i32,
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
        self.service(create_new_user)
            .service(insert_new_user)
            .service(get_all_user)
            .service(get_all_active_user)
            .service(upsert_user)
            .service(get_by_user_id)
    }
}

#[utoipa::path(
    post,
    tag = TAG_NAME,
    path = "/user-management/create-new-user",
    responses(
        (status = 200, description = "success"),
        (status = NOT_FOUND, description = "not found")
    ),
    request_body(
        content =  request_model::CreateNewUser,
        content_type =  "application/json",
    )
)]
#[post("/user-management/create-new-user")]
pub async fn create_new_user(request: Json<request_model::CreateNewUser>) -> impl Responder {
    let result = USER_MANAGEMENT_COMMAND
        .lock()
        .await
        .create_new_user(request.0.username)
        .await;

    HttpResponse::Ok().json(result)
}

#[utoipa::path(
    post,
    tag = TAG_NAME,
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
    let result = USER_MANAGEMENT_COMMAND
        .lock()
        .await
        .insert_new_user(request.new_user.clone())
        .await;

    HttpResponse::Ok().json(result)
}

#[utoipa::path(
    get,
    tag = TAG_NAME,
    path = "/user-management/get-all-user",
    responses(
        (status = 200, description = "success"),
        (status = NOT_FOUND, description = "not found")
    )
)]
#[get("/user-management/get-all-user")]
pub async fn get_all_user() -> impl Responder {
    let result = USER_MANAGEMENT_COMMAND.lock().await.get_all_user().await;

    HttpResponse::Ok().json(result)
}

#[utoipa::path(
    get,
    tag = TAG_NAME,
    path = "/user-management/get-all-active-user",
    responses(
        (status = 200, description = "success"),
        (status = NOT_FOUND, description = "not found")
    )
)]
#[get("/user-management/get-all-active-user")]
pub async fn get_all_active_user() -> impl Responder {
    let result = USER_MANAGEMENT_COMMAND
        .lock()
        .await
        .get_all_active_user()
        .await;

    HttpResponse::Ok().json(result)
}

#[utoipa::path(
    post,
    tag = TAG_NAME,
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
    let result = USER_MANAGEMENT_COMMAND
        .lock()
        .await
        .upsert_user(request.user.clone())
        .await;

    HttpResponse::Ok().json(result)
}

#[utoipa::path(
    post,
    tag = TAG_NAME,
    path = "/user-management/get-by-user-id",
    responses(
        (status = 200, description = "success"),
        (status = NOT_FOUND, description = "not found")
    ),
    request_body(
        content =  request_model::UserIdRequest ,
        content_type =  "application/json",
    )
)]
#[post("/user-management/get-by-user-id")]
pub async fn get_by_user_id(request: Json<request_model::UserIdRequest>) -> impl Responder {
    let result = USER_MANAGEMENT_COMMAND
        .lock()
        .await
        .get_by_user_id(request.user_id)
        .await;

    HttpResponse::Ok().json(result)
}
