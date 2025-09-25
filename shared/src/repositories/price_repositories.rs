use ams_entity::{prelude::*, price_history_table, user_table};
use sea_orm::{entity::*, prelude::async_trait::async_trait, query::*};

use crate::repositories::get_sql_connection_trait::GetSqlConnectionTrait;

#[async_trait]
pub trait AdditionalPriceHistoryTableMethodTrait {
    async fn get_latest_price() -> price_history_table::Model;
}

#[async_trait]
impl AdditionalPriceHistoryTableMethodTrait for PriceHistoryTable {
    async fn get_latest_price() -> price_history_table::Model {
        let conn = PriceHistoryTable::get_connection().await;

        let latest_price = PriceHistoryTable::find()
            .order_by_desc(price_history_table::Column::CreatedDate)
            .one(conn)
            .await
            .unwrap()
            .expect("price is empty, please seed price!!");

        latest_price
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn name() {
        todo!();
    }
}
