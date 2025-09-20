use ams_macro::GenerateTableEnum;
use chrono::NaiveDateTime;
use sea_orm_migration::prelude::*;

#[derive(GenerateTableEnum, Debug)]
#[warn(dead_code)]
pub struct MoneyHistory {
    pub id: u32,
    pub user_id: u32,
    pub date: NaiveDateTime,
    pub money_amount: i64,
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
                    .table(MoneyHistoryTable::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MoneyHistoryTable::Id)
                            .big_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(MoneyHistoryTable::UserId).big_unsigned())
                    .col(ColumnDef::new(MoneyHistoryTable::Date).date_time())
                    .col(ColumnDef::new(MoneyHistoryTable::MoneyAmount).big_integer())
                    .col(ColumnDef::new(MoneyHistoryTable::Description).big_integer())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(MoneyHistoryTable::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await
    }
}
