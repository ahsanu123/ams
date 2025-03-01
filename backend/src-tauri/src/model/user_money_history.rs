use crate::migration::migration_trait::Migrationable;
use chrono::NaiveDate;
use custom_macro::{GenerateDieselTable, GenerateTableEnum};
use diesel::{prelude::Insertable, Selectable};
use sea_query::{ColumnDef, Iden, Table};
use std::fmt::{self, *};

// UserMoneyHistory
// is struct to save when user add or Substract money,
// each UserMoneyHistory have action from UserMoneyAction
// so there is some description on each history

pub enum UserMoneyAction {
    Add,
    Substract,
}

impl Display for UserMoneyAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let action = match self {
            UserMoneyAction::Add => "Add",
            UserMoneyAction::Substract => "Substract",
        };
        write!(f, "{}", action)
    }
}

#[derive(Insertable)]
#[diesel(table_name = usermoneyhistory_table)]
pub struct UserMoneyHistoryNoId {
    pub date: NaiveDate,
    pub money_amount: f64,
    pub action: String,
}

#[derive(GenerateTableEnum, GenerateDieselTable, Selectable)]
#[diesel(table_name = usermoneyhistory_table)]
pub struct UserMoneyHistory {
    pub id: i32,
    pub date: NaiveDate,
    pub money_amount: f64,
    pub action: String,
}

impl Migrationable for UserMoneyAction {
    fn get_up_migration(builder: impl sea_query::SchemaBuilder) -> String {
        Table::create()
            .table(UserMoneyHistoryTable::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(UserMoneyHistoryTable::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new(UserMoneyHistoryTable::Date).date_time())
            .col(ColumnDef::new(UserMoneyHistoryTable::MoneyAmount).float())
            .col(ColumnDef::new(UserMoneyHistoryTable::Action).text())
            .build(builder)
    }

    fn get_down_migration(builder: impl sea_query::SchemaBuilder) -> String {
        Table::drop()
            .table(UserMoneyHistoryTable::Table)
            .if_exists()
            .build(builder)
    }
}
