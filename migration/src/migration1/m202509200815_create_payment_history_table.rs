use super::m202509200753_create_user_table::UserTable;
use ams_macro::GenerateTableEnum;
use chrono::NaiveDateTime;
use sea_orm_migration::prelude::*;

const PAYMENT_HISTORY_USER_ID_NAME: &str = "FK_payment_history_user_id";

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
        let payment_history_user_id = &mut ForeignKey::create()
            .name(PAYMENT_HISTORY_USER_ID_NAME)
            .from_tbl(PaymentHistoryTable::Table)
            .from_col(PaymentHistoryTable::UserId)
            .to_tbl(UserTable::Table)
            .to_col(UserTable::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();
        manager
            .create_table(
                Table::create()
                    .table(PaymentHistoryTable::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PaymentHistoryTable::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PaymentHistoryTable::UserId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PaymentHistoryTable::Date)
                            .date_time()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PaymentHistoryTable::BillAmount)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PaymentHistoryTable::InitialMoney)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PaymentHistoryTable::EndMoney)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PaymentHistoryTable::AddedMoney)
                            .big_integer()
                            .not_null(),
                    )
                    .foreign_key(payment_history_user_id)
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
