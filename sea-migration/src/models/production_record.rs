use ams_macro::GenerateTableEnum;
use chrono::NaiveDate;
use sea_orm_migration::{async_trait::async_trait, prelude::*, schema::*};
use sea_query::Table;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "migrating product record"
    }
}

#[allow(dead_code)]
#[derive(GenerateTableEnum)]
pub struct ProductionRecord {
    pub id: i64,
    pub amount: i32,
    pub date: NaiveDate,
}

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ProductionRecordTable::Table)
                    .if_not_exists()
                    .col(pk_auto(ProductionRecordTable::Id))
                    .col(integer(ProductionRecordTable::Amount))
                    .col(date(ProductionRecordTable::Date))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProductionRecordTable::Table).to_owned())
            .await
    }
}
