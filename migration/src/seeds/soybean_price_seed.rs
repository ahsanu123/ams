use ams_entity::{prelude::*, soybean_price_history_table};
use chrono::Local;
use sea_orm_migration::{
    prelude::*,
    sea_orm::{
        ActiveValue::{NotSet, Set},
        EntityTrait,
        entity::*,
    },
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let soybean_default_price = SoybeanPriceHistoryTable::find_by_id(1)
            .one(db)
            .await
            .unwrap();

        if !soybean_default_price.is_some() {
            let _ = soybean_price_history_table::ActiveModel {
                id: NotSet,
                date: Set(Local::now().naive_local()),
                price: Set(15000),
            }
            .insert(db)
            .await;
        }

        Ok(())
    }
}
