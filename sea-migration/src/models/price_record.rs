use ams_macro::GenerateTableEnum;
use chrono::NaiveDate;
use sea_orm_migration::{async_trait::async_trait, prelude::*, schema::*};
use sea_query::Table;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "migrating price record"
    }
}

#[allow(dead_code)]
#[derive(GenerateTableEnum)]
pub struct PriceRecord {
    pub id: i64,
    pub price: f64,
    pub date: NaiveDate,
    pub description: String,
}

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PriceRecordTable::Table)
                    .if_not_exists()
                    .col(pk_auto(PriceRecordTable::Id))
                    .col(float(PriceRecordTable::Price))
                    .col(date(PriceRecordTable::Date))
                    .col(string(PriceRecordTable::Description))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PriceRecordTable::Table).to_owned())
            .await
    }
}
