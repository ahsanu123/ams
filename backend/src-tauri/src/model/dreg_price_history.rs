use crate::migration::migration_trait::Migrationable;
use chrono::NaiveDate;
use custom_macro::{GenerateDieselTable, GenerateTableEnum};
use diesel::{prelude::Insertable, Selectable};
use sea_query::{ColumnDef, Iden, Table};

#[derive(Insertable)]
#[diesel(table_name = pricehistory_table)]
pub struct PriceHistoryNoId {
    pub price: f64,
    pub changed_date: NaiveDate,
    pub description: String,
}

#[derive(GenerateTableEnum, GenerateDieselTable, Selectable)]
#[diesel(table_name = pricehistory_table)]
pub struct PriceHistory {
    pub id: i32,
    pub price: f64,
    pub changed_date: NaiveDate,
    pub description: String,
}

impl Migrationable for PriceHistory {
    fn get_up_migration(builder: impl sea_query::SchemaBuilder) -> String {
        Table::create()
            .table(PriceHistoryTable::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(PriceHistoryTable::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new(PriceHistoryTable::Price).float())
            .col(ColumnDef::new(PriceHistoryTable::ChangedDate).date_time())
            .col(ColumnDef::new(PriceHistoryTable::Description).text())
            .build(builder)
    }

    fn get_down_migration(builder: impl sea_query::SchemaBuilder) -> String {
        Table::drop()
            .table(PriceHistoryTable::Table)
            .if_exists()
            .build(builder)
    }
}
