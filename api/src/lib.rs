use crate::controllers::ApiDoc;
use actix_cors::Cors;
use actix_files::Files;
use actix_web::{App, HttpServer, middleware::Logger};
use ams_shared::helper::ENV_VAR;
use utoipa::OpenApi;
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::{Config, SwaggerUi};

pub mod controllers;

pub fn start_server_blocking() -> std::io::Result<()> {
    actix_web::rt::System::new().block_on(async { start_server().await })
}

pub async fn start_server() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_origin()
            .allow_any_method();

        let static_path = ENV_VAR.get().unwrap().static_file_path.clone();

        App::new()
            .wrap(cors)
            .wrap(Logger::new("%a %{User-Agent}i"))
            .into_utoipa_app()
            .openapi(ApiDoc::openapi())
            .openapi_service(|api| {
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api/openapi.json", api)
                    .config(Config::default().try_it_out_enabled(true))
            })
            .into_app()
            // register all endpoint here to be able to accessed
            // .register_customer_endpoints()
            // .register_dregs_price_endpoints()
            // .register_payment_endpoints()
            // .register_taking_record_endpoints()
            // .register_user_management_endpoints()
            // .register_make_payment_page_endpoints()
            //  static file service
            .service(Files::new("/", static_path).index_file("index.html"))
    })
    .bind(("127.0.0.1", 9090))?
    .run()
    .await
}
