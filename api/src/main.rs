use ams_shared::init_environment_variable;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    init_environment_variable();
    ams_api::start_server().await
}
