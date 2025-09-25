use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use ams_api::endpoints::{
    informations_endpoint::information_get_connection_string,
    taking_record_endpoint::taking_record_add_new_taking_record,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(taking_record_add_new_taking_record)
            .service(information_get_connection_string)
    })
    .bind(("127.0.0.1", 9090))?
    .run()
    .await
}
