use ams_entity::{prelude::*, soybean_price_history_table};
use sea_orm::{entity::*, prelude::async_trait::async_trait, query::*};

use crate::repositories::database_connection::get_database_connection;

pub trait AdditionalSoybeanPriceHistoryTableMethodTrait {
    async fn get_latest_price(&mut self) -> soybean_price_history_table::Model;
}

#[derive(Default)]
pub struct SoybeanPriceRepository {}

impl AdditionalSoybeanPriceHistoryTableMethodTrait for SoybeanPriceRepository {
    async fn get_latest_price(&mut self) -> soybean_price_history_table::Model {
        let conn = get_database_connection().await;

        let latest_price = SoybeanPriceHistoryTable::find()
            .order_by_desc(soybean_price_history_table::Column::Date)
            .one(conn)
            .await
            .unwrap()
            .expect("soybean price is empty, please seed price!!");

        latest_price
    }
}
