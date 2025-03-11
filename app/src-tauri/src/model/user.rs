use crate::migration::migration_trait::Migrationable;
use custom_macro::{GenerateDieselTable, GenerateTableEnum};
use diesel::prelude::*;
use sea_query::{ColumnDef, Iden, Table};

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = user_table)]
pub struct UserNoId {
    pub username: String,
    pub is_active: bool,
    pub money: f64,
    pub bill: f64,
}

#[derive(GenerateTableEnum, GenerateDieselTable, Selectable, Queryable, Debug, PartialEq)]
#[diesel(table_name = user_table)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub is_active: bool,
    pub money: f64,
    pub bill: f64,
}

impl From<&User> for UserNoId {
    fn from(value: &User) -> Self {
        UserNoId {
            username: value.username.clone(),
            is_active: value.is_active,
            money: value.money,
            bill: value.bill,
        }
    }
}

impl Migrationable for User {
    fn get_up_migration(builder: impl sea_query::SchemaBuilder) -> String {
        Table::create()
            .table(UserTable::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(UserTable::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new(UserTable::Username).text())
            .col(ColumnDef::new(UserTable::IsActive).boolean())
            .col(ColumnDef::new(UserTable::Money).float())
            .col(ColumnDef::new(UserTable::Bill).float())
            .build(builder)
    }

    fn get_down_migration(builder: impl sea_query::SchemaBuilder) -> String {
        Table::drop()
            .table(UserTable::Table)
            .if_exists()
            .build(builder)
    }
}
