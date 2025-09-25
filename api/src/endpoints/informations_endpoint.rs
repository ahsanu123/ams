use actix_web::{HttpResponse, Responder, get, post};
use ams_shared::commands::information_command::get_connection_string;

#[get("/information/connection-string")]
pub async fn information_get_connection_string() -> impl Responder {
    let conn_string = get_connection_string().await;

    HttpResponse::Ok().body(conn_string)
}
