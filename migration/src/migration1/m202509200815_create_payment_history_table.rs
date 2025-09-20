use ams_macro::GenerateTableEnum;
use chrono::NaiveDateTime;
use sea_orm_migration::prelude::*;

#[derive(GenerateTableEnum, Debug)]
#[warn(dead_code)]
pub struct PaymentHistory {
    pub id: u32,
    pub user_id: u32,
    pub date: NaiveDateTime,
    pub bill_amount: i64,
    pub initial_money: i64,
    pub end_money: i64,
    pub added_money: i64,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PaymentHistoryTable::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PaymentHistoryTable::Id)
                            .big_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(PaymentHistoryTable::UserId).big_unsigned())
                    .col(ColumnDef::new(PaymentHistoryTable::Date).date_time())
                    .col(ColumnDef::new(PaymentHistoryTable::BillAmount).big_integer())
                    .col(ColumnDef::new(PaymentHistoryTable::InitialMoney).big_integer())
                    .col(ColumnDef::new(PaymentHistoryTable::EndMoney).big_integer())
                    .col(ColumnDef::new(PaymentHistoryTable::AddedMoney).big_integer())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(PaymentHistoryTable::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await
    }
}
