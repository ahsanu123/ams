use ams_macro::GenerateTableEnum;
use chrono::{NaiveDate, NaiveDateTime};
use sea_orm_migration::prelude::*;

#[derive(GenerateTableEnum, Debug)]
#[warn(dead_code)]
pub struct ProductionRecord {
    pub id: u32,
    pub date: NaiveDateTime,
    pub amount: i64,
    pub dreg_price_id: u32,
    pub soybean_price_id: u32,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ProductionRecordTable::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProductionRecordTable::Id)
                            .big_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ProductionRecordTable::Date).date_time())
                    .col(ColumnDef::new(ProductionRecordTable::Amount).big_integer())
                    .col(ColumnDef::new(ProductionRecordTable::DregPriceId).big_unsigned())
                    .col(ColumnDef::new(ProductionRecordTable::SoybeanPriceId).big_unsigned())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(ProductionRecordTable::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await
    }
}
