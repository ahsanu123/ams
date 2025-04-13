use crate::migration::migration_trait::Migrationable;
use ams_macro::{GenerateDieselTable, GenerateTableEnum};
use chrono::NaiveDate;
use diesel::{prelude::Insertable, Selectable};
use sea_query::{ColumnDef, Iden, Table};
use std::fmt::{self, *};

pub enum MoneyRecordAction {
    Add,
    Substract,
}

impl Display for MoneyRecordAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let action = match self {
            MoneyRecordAction::Add => "Add",
            MoneyRecordAction::Substract => "Substract",
        };
        write!(f, "{}", action)
    }
}

#[derive(Insertable)]
#[diesel(table_name = moneyrecord_table)]
pub struct MoneyRecordNoId {
    pub date: NaiveDate,
    pub money_amount: f64,
    pub bill_amount: f64,
    pub created_date: NaiveDate,
    pub action: String,
}

#[derive(GenerateTableEnum, GenerateDieselTable, Selectable)]
#[diesel(table_name = moneyrecord_table)]
pub struct MoneyRecord {
    pub id: i32,
    pub date: NaiveDate,
    pub money_amount: f64,
    pub bill_amount: f64,
    pub created_date: NaiveDate,
    pub action: String,
}

impl Migrationable for MoneyRecord {
    fn get_up_migration(builder: impl sea_query::SchemaBuilder) -> String {
        Table::create()
            .table(MoneyRecordTable::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(MoneyRecordTable::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new(MoneyRecordTable::Date).date_time())
            .col(ColumnDef::new(MoneyRecordTable::MoneyAmount).float())
            .col(ColumnDef::new(MoneyRecordTable::BillAmount).float())
            .col(ColumnDef::new(MoneyRecordTable::CreatedDate).float())
            .col(ColumnDef::new(MoneyRecordTable::Action).text())
            .build(builder)
    }

    fn get_down_migration(builder: impl sea_query::SchemaBuilder) -> String {
        Table::drop()
            .table(MoneyRecordTable::Table)
            .if_exists()
            .build(builder)
    }
}
