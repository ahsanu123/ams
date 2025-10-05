pub mod customer_endpoints;
pub mod dreg_price_endpoint;
pub mod payment_history_endpoint;
pub mod taking_record_endpoint;
pub mod user_management_enpoint;

use utoipa::OpenApi;

// register all endpoint here to be shown in swagger-ui
#[derive(OpenApi)]
#[openapi(
    paths(
        // customer_endpoints
        crate::endpoints::customer_endpoints::add_money,
        crate::endpoints::customer_endpoints::get_all_user_money_history,
        // dreg_price_endpoint
        crate::endpoints::dreg_price_endpoint::get_latest_dreg_price,
        crate::endpoints::dreg_price_endpoint::update_dreg_price,
        crate::endpoints::dreg_price_endpoint::get_all_dreg_price,
        //payment_history_endpoint
        crate::endpoints::payment_history_endpoint::get_payment_record_by_user_id,
        crate::endpoints::payment_history_endpoint::get_month_summary,
        crate::endpoints::payment_history_endpoint::get_payment_record_by_user_id_and_month,
        crate::endpoints::payment_history_endpoint::get_month_summary_by_user_id,
        crate::endpoints::payment_history_endpoint::update_payment_record,
        crate::endpoints::payment_history_endpoint::update_bulk_payment_record,
        // taking_record_endpoint
        crate::endpoints::taking_record_endpoint::add_new_taking_record,
        crate::endpoints::taking_record_endpoint::get_taking_record_by_user_id,
        crate::endpoints::taking_record_endpoint::upsert_taking_record,
        crate::endpoints::taking_record_endpoint::get_taking_record_by_month,
        crate::endpoints::taking_record_endpoint::get_taking_record_by_user_id_and_month,
        // user_management_enpoint
        crate::endpoints::user_management_enpoint::insert_new_user,
        crate::endpoints::user_management_enpoint::get_all_user,
        crate::endpoints::user_management_enpoint::get_all_active_user,
        crate::endpoints::user_management_enpoint::upsert_user,
    ),
    components(schemas())
)]
pub struct ApiDoc;
