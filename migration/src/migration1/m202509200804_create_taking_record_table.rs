use super::m202509200753_create_user_table::UserTable;
use super::m202509200811_create_price_history_table::PriceHistoryTable;
use ams_macro::GenerateTableEnum;
use chrono::NaiveDateTime;
use sea_orm_migration::prelude::*;

const TAKING_RECORD_USER_ID_NAME: &str = "FK_taking_record_user_id";
const TAKING_RECORD_PRICE_ID_NAME: &str = "FK_taking_record_price_id";

#[derive(GenerateTableEnum, Debug)]
#[warn(dead_code)]
pub struct TakingRecord {
    pub id: u32,
    pub user_id: u32,
    pub price_id: u32,
    pub amount: i32,
    pub production_date: NaiveDateTime,
    pub taken_date: NaiveDateTime,
    pub description: String,
    pub is_paid: bool,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let taking_record_user_id = &mut ForeignKey::create()
            .name(TAKING_RECORD_USER_ID_NAME)
            .from_tbl(TakingRecordTable::Table)
            .from_col(TakingRecordTable::UserId)
            .to_tbl(UserTable::Table)
            .to_col(UserTable::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();

        let taking_record_price_id = &mut ForeignKey::create()
            .name(TAKING_RECORD_PRICE_ID_NAME)
            .from_tbl(TakingRecordTable::Table)
            .from_col(TakingRecordTable::PriceId)
            .to_tbl(PriceHistoryTable::Table)
            .to_col(PriceHistoryTable::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table(TakingRecordTable::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TakingRecordTable::Id)
                            .big_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TakingRecordTable::UserId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TakingRecordTable::PriceId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TakingRecordTable::Amount)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TakingRecordTable::ProductionDate)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TakingRecordTable::TakenDate)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TakingRecordTable::Description)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TakingRecordTable::IsPaid)
                            .boolean()
                            .not_null(),
                    )
                    .foreign_key(taking_record_price_id)
                    .foreign_key(taking_record_user_id)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(TakingRecordTable::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await
    }
}
