use ams_macro::GenerateTableEnum;
use chrono::NaiveDate;
use sea_orm_migration::{async_trait::async_trait, prelude::*, schema::*};
use sea_query::Table;

#[derive(DeriveMigrationName, GenerateTableEnum)]
pub struct PaymentTaking {
    pub id: i64,
    pub taking_id: i64,
    pub payment_id: i64,
}

#[async_trait]
impl MigrationTrait for PaymentTaking {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PaymentTakingTable::Table)
                    .if_not_exists()
                    .col(pk_auto(PaymentTakingTable::Id))
                    .col(float(PaymentTakingTable::TakingId))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PaymentTakingTable::Table).to_owned())
            .await
    }
}
