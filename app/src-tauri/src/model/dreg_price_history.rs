use crate::migration::migration_trait::Migrationable;
use ams_macro::{GenerateDieselTable, GenerateTableEnum};
use chrono::NaiveDate;
use diesel::{prelude::Insertable, Selectable};
use sea_query::{ColumnDef, Iden, Table};

#[derive(Insertable)]
#[diesel(table_name = dregprice_table)]
pub struct DregPriceNoId {
    pub price: f64,
    pub changed_date: NaiveDate,
    pub description: String,
}

#[derive(GenerateTableEnum, GenerateDieselTable, Selectable)]
#[diesel(table_name = dregprice_table)]
pub struct DregPrice {
    pub id: i32,
    pub price: f64,
    pub changed_date: NaiveDate,
    pub description: String,
}

impl Migrationable for DregPrice {
    fn get_up_migration(builder: impl sea_query::SchemaBuilder) -> String {
        Table::create()
            .table(DregPriceTable::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(DregPriceTable::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new(DregPriceTable::Price).float())
            .col(ColumnDef::new(DregPriceTable::ChangedDate).date_time())
            .col(ColumnDef::new(DregPriceTable::Description).text())
            .build(builder)
    }

    fn get_down_migration(builder: impl sea_query::SchemaBuilder) -> String {
        Table::drop()
            .table(DregPriceTable::Table)
            .if_exists()
            .build(builder)
    }
}
