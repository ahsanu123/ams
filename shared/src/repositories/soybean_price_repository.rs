use ams_entity::{prelude::*, price_history_table, soybean_price_history_table};
use sea_orm::{entity::*, prelude::async_trait::async_trait, query::*};

use crate::repositories::get_sql_connection_trait::GetSqlConnectionTrait;

#[async_trait]
pub trait AdditionalSoybeanPriceHistoryTableMethodTrait {
    async fn get_latest_price() -> soybean_price_history_table::Model;
}

#[async_trait]
impl AdditionalSoybeanPriceHistoryTableMethodTrait for SoybeanPriceHistoryTable {
    async fn get_latest_price() -> soybean_price_history_table::Model {
        let conn = SoybeanPriceHistoryTable::get_connection().await;

        let latest_price = SoybeanPriceHistoryTable::find()
            .order_by_desc(soybean_price_history_table::Column::Date)
            .one(conn)
            .await
            .unwrap()
            .expect("soybean price is empty, please seed price!!");

        latest_price
    }
}
