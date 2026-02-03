#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    ams_api::start_server().await
}
