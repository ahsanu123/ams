use super::m202509200811_create_price_history_table::PriceHistoryTable;
use super::m202509200818_create_soybean_price_history_table::SoybeanPriceHistoryTable;
use ams_macro::GenerateTableEnum;
use chrono::NaiveDateTime;
use sea_orm_migration::prelude::*;

const PRODUCTION_RECORD_PRICE_ID_NAME: &str = "FK_production_record_price_id";
const PRODUCTION_RECORD_SOYBEAN_PRICE_ID_NAME: &str = "FK_production_record_soybean_price_id";

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
        let production_record_user_id = &mut ForeignKey::create()
            .name(PRODUCTION_RECORD_PRICE_ID_NAME)
            .from_tbl(ProductionRecordTable::Table)
            .from_col(ProductionRecordTable::DregPriceId)
            .to_tbl(PriceHistoryTable::Table)
            .to_col(PriceHistoryTable::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();

        let production_record_price_id = &mut ForeignKey::create()
            .name(PRODUCTION_RECORD_SOYBEAN_PRICE_ID_NAME)
            .from_tbl(ProductionRecordTable::Table)
            .from_col(ProductionRecordTable::SoybeanPriceId)
            .to_tbl(SoybeanPriceHistoryTable::Table)
            .to_col(SoybeanPriceHistoryTable::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();
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
                    .col(
                        ColumnDef::new(ProductionRecordTable::Date)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProductionRecordTable::Amount)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProductionRecordTable::DregPriceId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProductionRecordTable::SoybeanPriceId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .foreign_key(production_record_price_id)
                    .foreign_key(production_record_user_id)
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
