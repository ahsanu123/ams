use super::user::*;
use crate::migration::migration_trait::Migrationable;
use ams_macro::{GenerateDieselTable, GenerateTableEnum};
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel::Selectable;
use sea_query::ForeignKey;
use sea_query::ForeignKeyAction;
use sea_query::{ColumnDef, Iden, SchemaBuilder, Table};

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = record_table)]
pub struct RecordNoId {
    pub user_id: i32,
    pub paid: bool,
    pub production_date: NaiveDate,
    pub taken_date: NaiveDate,
    pub price: f64,
    pub amount: i32,
    pub description: String,
}

impl From<&Record> for RecordNoId {
    fn from(value: &Record) -> Self {
        RecordNoId {
            user_id: value.user_id,
            paid: value.paid,
            production_date: value.production_date,
            taken_date: value.taken_date,
            price: value.price,
            amount: value.amount,
            description: value.description.clone(),
        }
    }
}

// with as AsChangeset here
// and optional type
// we can partially update table value,
// i think it will get benefit if
// table have many column,
// but right now its simple application
// maybe in future i will implement it :)
#[derive(AsChangeset)]
#[diesel(table_name = record_table)]
pub struct RecordChangeSet {
    pub user_id: Option<i32>,
    pub paid: Option<bool>,
    pub production_date: Option<NaiveDate>,
    pub taken_date: Option<NaiveDate>,
    pub price: Option<f64>,
    pub amount: Option<i32>,
    pub description: Option<String>,
}

#[derive(GenerateTableEnum, GenerateDieselTable, Selectable, Queryable, Debug, PartialEq)]
#[diesel(table_name = record_table)]
pub struct Record {
    pub id: i32,
    pub user_id: i32,
    pub paid: bool,
    pub production_date: NaiveDate,
    pub taken_date: NaiveDate,
    pub price: f64,
    pub amount: i32,
    pub description: String,
}

diesel::joinable!(record_table -> user_table(user_id));
diesel::allow_tables_to_appear_in_same_query!(record_table, user_table);

impl Migrationable for Record {
    fn get_up_migration(builder: impl SchemaBuilder) -> String {
        Table::create()
            .table(RecordTable::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(RecordTable::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new(RecordTable::UserId).integer())
            .col(ColumnDef::new(RecordTable::Paid).boolean())
            .col(ColumnDef::new(RecordTable::ProductionDate).date_time())
            .col(ColumnDef::new(RecordTable::TakenDate).date_time())
            .col(ColumnDef::new(RecordTable::Price).double())
            .col(ColumnDef::new(RecordTable::Amount).integer())
            .col(ColumnDef::new(RecordTable::Description).text())
            .foreign_key(
                ForeignKey::create()
                    .name("FK_ProductToUser")
                    .from(RecordTable::Table, RecordTable::UserId)
                    .to(UserTable::Table, UserTable::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .build(builder)
    }

    fn get_down_migration(builder: impl SchemaBuilder) -> String {
        Table::drop()
            .table(RecordTable::Table)
            .if_exists()
            .build(builder)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::helper::sql_connection_helper::create_connection;
    use sea_query::{PostgresQueryBuilder, SqliteQueryBuilder};

    #[test]
    fn test_product_up_migration() {
        let product_sqlite = Record::get_up_migration(SqliteQueryBuilder);
        let product_postgresql = Record::get_up_migration(PostgresQueryBuilder);

        println!("Sqlite -> \n{product_sqlite}");
        println!("Postgresql -> \n{product_postgresql}");
    }

    #[test]
    fn test_insert_product() {
        let conn = &mut create_connection();
        let record = RecordNoId {
            user_id: 2,
            paid: false,
            production_date: NaiveDate::from_ymd_opt(2025, 2, 3).unwrap(),
            taken_date: NaiveDate::from_ymd_opt(2025, 2, 3).unwrap(),
            price: 11000.0,
            amount: 2,
            description: String::from("taken was happen!!"),
        };

        let _result = diesel::insert_into(super::record_table::table)
            .values(&record)
            .execute(conn)
            .expect("error when inserting data");
    }
}
