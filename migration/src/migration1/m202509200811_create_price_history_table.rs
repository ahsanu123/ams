use ams_macro::GenerateTableEnum;
use chrono::NaiveDateTime;
use sea_orm_migration::prelude::*;

#[derive(GenerateTableEnum, Debug)]
#[warn(dead_code)]
pub struct PriceHistory {
    pub id: u32,
    pub created_date: NaiveDateTime,
    pub price: i64,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PriceHistoryTable::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PriceHistoryTable::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PriceHistoryTable::CreatedDate)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PriceHistoryTable::Price)
                            .big_integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(PriceHistoryTable::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await
    }
}
