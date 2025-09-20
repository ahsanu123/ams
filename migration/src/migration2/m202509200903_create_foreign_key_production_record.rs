use crate::migration1::m202509200754_create_production_record_table::ProductionRecordTable;
use crate::migration1::m202509200811_create_price_history_table::PriceHistoryTable;
use crate::migration1::m202509200818_create_soybean_price_history_table::SoybeanPriceHistoryTable;
use sea_orm_migration::prelude::*;

const PRODUCTION_RECORD_PRICE_ID_NAME: &str = "FK_production_record_price_id";
const PRODUCTION_RECORD_SOYBEAN_PRICE_ID_NAME: &str = "FK_production_record_soybean_price_id";

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let production_record_user_id = TableForeignKey::new()
            .name(PRODUCTION_RECORD_PRICE_ID_NAME)
            .from_tbl(ProductionRecordTable::Table)
            .from_col(ProductionRecordTable::DregPriceId)
            .to_tbl(PriceHistoryTable::Table)
            .to_col(PriceHistoryTable::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();

        let production_record_price_id = TableForeignKey::new()
            .name(PRODUCTION_RECORD_SOYBEAN_PRICE_ID_NAME)
            .from_tbl(ProductionRecordTable::Table)
            .from_col(ProductionRecordTable::SoybeanPriceId)
            .to_tbl(SoybeanPriceHistoryTable::Table)
            .to_col(SoybeanPriceHistoryTable::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .alter_table(
                Table::alter()
                    .table(ProductionRecordTable::Table)
                    .add_foreign_key(&production_record_user_id)
                    .add_foreign_key(&production_record_price_id)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let production_record_price_id_drop_statement = ForeignKey::drop()
            .name(PRODUCTION_RECORD_PRICE_ID_NAME)
            .table(ProductionRecordTable::Table)
            .to_owned();

        let taking_record_price_soybean_id_drop_statement = ForeignKey::drop()
            .name(PRODUCTION_RECORD_SOYBEAN_PRICE_ID_NAME)
            .table(ProductionRecordTable::Table)
            .to_owned();

        manager
            .drop_foreign_key(production_record_price_id_drop_statement)
            .await?;

        manager
            .drop_foreign_key(taking_record_price_soybean_id_drop_statement)
            .await?;
        Ok(())
    }
}
