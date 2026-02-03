use crate::repositories::{
    database_connection::get_database_connection,
    price_repositories::{AdditionalPriceHistoryTableMethodTrait, PriceRepository},
    soybean_price_repository::{
        AdditionalSoybeanPriceHistoryTableMethodTrait, SoybeanPriceRepository,
    },
};
use ams_entity::production_record_table;
use sea_orm::entity::*;

pub trait ProductionRecordCommandTrait {
    async fn insert_production_record(&mut self, record: production_record_table::Model) -> i32;
}

pub struct ProductionRecordCommand {
    price_repository: PriceRepository,
    soybean_price_repository: SoybeanPriceRepository,
}

impl Default for ProductionRecordCommand {
    fn default() -> Self {
        Self {
            price_repository: PriceRepository::default(),
            soybean_price_repository: SoybeanPriceRepository::default(),
        }
    }
}

impl ProductionRecordCommandTrait for ProductionRecordCommand {
    async fn insert_production_record(&mut self, record: production_record_table::Model) -> i32 {
        let conn = get_database_connection().await;
        let latest_dreg_price = self.price_repository.get_latest_price().await;
        let latest_soybean_price = self.soybean_price_repository.get_latest_price().await;

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
