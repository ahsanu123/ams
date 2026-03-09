use crate::controllers::{
    balance_controller::BalanceServiceExtensionTrait,
    billing_controller::BillingServiceExtensionTrait,
    customer_management_controller::CustomerManagementServiceExtensionTrait,
    modified_docs_with_security_schemes, price_controller::PriceControllerServiceExtensionTrait,
    retrieve_data_controller::RetrieveDataServiceExtensionTrait,
};
use actix_cors::Cors;
use actix_files::Files;
use actix_web::{App, HttpServer, middleware::Logger};
use ams_shared::helper::ENV_VAR;
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::{Config, SwaggerUi};

pub mod controllers;
pub mod extractors;

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
            .openapi(modified_docs_with_security_schemes())
            .openapi_service(|api| {
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api/openapi.json", api)
                    .config(Config::default().try_it_out_enabled(true))
            })
            .into_app()
            .register_retrieve_data_controller()
            .register_customer_controller()
            .register_balance_controller()
            .register_billing_controller()
            .register_price_controller()
            //  static file service
            .service(Files::new("/", static_path).index_file("index.html"))
    })
    .bind(("127.0.0.1", 9090))?
    .run()
    .await
}
