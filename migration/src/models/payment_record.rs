use ams_macro::GenerateTableEnum;
use chrono::NaiveDate;
use sea_orm_migration::{async_trait::async_trait, prelude::*, schema::*};
use sea_query::Table;

#[derive(DeriveMigrationName, GenerateTableEnum)]
pub struct PaymentRecord {
    pub id: i64,
    pub user_id: i64,
    pub date: NaiveDate,
    pub paying_amount: f64,
}

#[async_trait]
impl MigrationTrait for PaymentRecord {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PaymentRecordTable::Table)
                    .if_not_exists()
                    .col(pk_auto(PaymentRecordTable::Id))
                    .col(integer(PaymentRecordTable::UserId))
                    .col(date(PaymentRecordTable::Date))
                    .col(float(PaymentRecordTable::PayingAmount))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PaymentRecordTable::Table).to_owned())
            .await
    }
}
