use ams_macro::GenerateTableEnum;
use chrono::NaiveDate;
use sea_orm_migration::{async_trait::async_trait, prelude::*, schema::*};
use sea_query::Table;

#[derive(DeriveMigrationName, GenerateTableEnum)]
pub struct TakingRecord {
    pub id: i64,
    pub amount: i32,
    pub date: NaiveDate,
    pub paid: bool,
}

#[async_trait]
impl MigrationTrait for TakingRecord {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TakingRecordTable::Table)
                    .if_not_exists()
                    .col(pk_auto(TakingRecordTable::Id))
                    .col(integer(TakingRecordTable::Amount))
                    .col(date(TakingRecordTable::Date))
                    .col(boolean(TakingRecordTable::Paid))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TakingRecordTable::Table).to_owned())
            .await
    }
}
