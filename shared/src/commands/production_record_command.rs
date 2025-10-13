use crate::repositories::{
    get_sql_connection_trait::GetSqlConnectionTrait,
    price_repositories::AdditionalPriceHistoryTableMethodTrait,
    soybean_price_repository::AdditionalSoybeanPriceHistoryTableMethodTrait,
};
use ams_entity::{
    prelude::{PriceHistoryTable, SoybeanPriceHistoryTable},
    production_record_table,
};
use sea_orm::{entity::*, prelude::async_trait};

#[async_trait::async_trait]
pub trait ProductionRecordCommandTrait {
    async fn insert_production_record(record: production_record_table::Model) -> i32;
}

pub struct ProductionRecordCommand;

#[async_trait::async_trait]
impl ProductionRecordCommandTrait for ProductionRecordCommand {
    async fn insert_production_record(record: production_record_table::Model) -> i32 {
        let conn = PriceHistoryTable::get_connection().await;
        let latest_dreg_price = PriceHistoryTable::get_latest_price().await;
        let latest_soybean_price = SoybeanPriceHistoryTable::get_latest_price().await;

        let insert_result = production_record_table::ActiveModel {
            id: NotSet,
            date: Set(record.date),
            amount: Set(record.amount),
            dreg_price_id: Set(latest_dreg_price.id as i64),
            soybean_price_id: Set(latest_soybean_price.id as i64),
        }
        .insert(conn)
        .await
        .unwrap();

        insert_result.id
    }
}
