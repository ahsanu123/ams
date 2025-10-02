use ams_entity::{prelude::*, price_history_table};
use chrono::Local;
use sea_orm_migration::{
    prelude::*,
    sea_orm::{
        ActiveValue::{NotSet, Set},
        EntityTrait,
        entity::*,
        query::*,
    },
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let default_price = PriceHistoryTable::find_by_id(1).one(db).await.unwrap();

        if !default_price.is_some() {
            let _ = price_history_table::ActiveModel {
                id: NotSet,
                created_date: Set(Local::now().naive_local()),
                price: Set(11000),
            }
            .insert(db)
            .await;
        }

        Ok(())
    }
}
