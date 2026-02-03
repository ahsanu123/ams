use crate::repositories::database_connection::get_database_connection;
use ams_entity::{prelude::*, price_history_table};
use sea_orm::{entity::*, query::*};

pub trait AdditionalPriceHistoryTableMethodTrait {
    async fn get_latest_price(&mut self) -> price_history_table::Model;
}

#[derive(Default)]
pub struct PriceRepository {}

impl AdditionalPriceHistoryTableMethodTrait for PriceRepository {
    async fn get_latest_price(&mut self) -> price_history_table::Model {
        let conn = get_database_connection().await;

        let latest_price = PriceHistoryTable::find()
            .order_by_desc(price_history_table::Column::CreatedDate)
            .one(conn)
            .await
            .unwrap()
            .expect("price is empty, please seed price!!");

        latest_price
    }
}
