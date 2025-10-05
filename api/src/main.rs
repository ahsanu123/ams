use actix_web::{App, HttpServer};
use ams_api::endpoints::{
    customer_endpoints::CustomerServiceExtensionTrait,
    dreg_price_endpoint::DregsPriceServiceExtensionTrait,
    payment_history_endpoint::PaymentServiceExtensionTrait,
    taking_record_endpoint::TakingRecordServiceExtensionTrait,
    user_management_enpoint::UserManagementServiceExtensionTrait,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .register_customer_endpoints()
            .register_dregs_price_endpoints()
            .register_payment_endpoints()
            .register_taking_record_endpoints()
            .register_user_management_endpoints()
    })
    .bind(("127.0.0.1", 9090))?
    .run()
    .await
}
