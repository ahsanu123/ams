use ams_macro::GenerateTableEnum;
use chrono::NaiveDateTime;
use sea_orm_migration::prelude::*;

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
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
                    .col(ColumnDef::new(TakingRecordTable::UserId).big_unsigned())
                    .col(ColumnDef::new(TakingRecordTable::PriceId).big_unsigned())
                    .col(ColumnDef::new(TakingRecordTable::Amount).big_unsigned())
                    .col(ColumnDef::new(TakingRecordTable::ProductionDate).date_time())
                    .col(ColumnDef::new(TakingRecordTable::TakenDate).date_time())
                    .col(ColumnDef::new(TakingRecordTable::Description).text())
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
