use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::Logger};
use ams_api::endpoints::ApiDoc;
use ams_api::endpoints::{
    customer_endpoints::CustomerServiceExtensionTrait,
    dreg_price_endpoint::DregsPriceServiceExtensionTrait,
    make_payment_page_endpoint::MakePaymentPageServiceExtensionTrait,
    payment_history_endpoint::PaymentServiceExtensionTrait,
    taking_record_endpoint::TakingRecordServiceExtensionTrait,
    user_management_enpoint::UserManagementServiceExtensionTrait,
};

use ams_shared::helper::ENV_VAR;
use ams_shared::helper::environment_variable::EnvironmentVariable;
use utoipa::OpenApi;
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::{Config, SwaggerUi};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let _ = ENV_VAR.set(EnvironmentVariable::new());

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_origin()
            .allow_any_method();

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
            .register_customer_endpoints()
            .register_dregs_price_endpoints()
            .register_payment_endpoints()
            .register_taking_record_endpoints()
            .register_user_management_endpoints()
            .register_make_payment_page_endpoints()
    })
    .bind(("127.0.0.1", 9090))?
    .run()
    .await
}
