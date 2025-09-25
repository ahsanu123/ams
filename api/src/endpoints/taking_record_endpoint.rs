use actix_web::{
    App, HttpResponse, HttpServer, Responder, get, post,
    web::{self, Json},
};
use ams_shared::commands::taking_record_command::*;
use serde::Deserialize;

// TODO: move this to new module
#[derive(Deserialize)]
struct AddNewTakingRecordRequest {
    pub user_id: i32,
    pub amount: i32,
}

#[post("/taking-record/add-new-taking-record")]
pub async fn taking_record_add_new_taking_record(
    request: Json<AddNewTakingRecordRequest>,
) -> impl Responder {
    let result = add_new_taking_record(request.user_id, request.amount).await;

    if result == 0 {
        return HttpResponse::NotFound().body(format!("user with id : {0} is not found!!", result));
    }

    HttpResponse::Ok().body("Hello world!")
}
