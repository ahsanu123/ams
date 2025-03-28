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
#[diesel(table_name = product_table)]
pub struct ProductNoId {
    pub user_id: i32,
    pub paid: bool,
    pub production_date: NaiveDate,
    pub taken_date: NaiveDate,
    pub price: f64,
    pub amount: i32,
    pub description: String,
}

impl From<&Product> for ProductNoId {
    fn from(value: &Product) -> Self {
        ProductNoId {
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
#[diesel(table_name = product_table)]
pub struct ProductChangeSet {
    pub user_id: Option<i32>,
    pub paid: Option<bool>,
    pub production_date: Option<NaiveDate>,
    pub taken_date: Option<NaiveDate>,
    pub price: Option<f64>,
    pub amount: Option<i32>,
    pub description: Option<String>,
}

#[derive(GenerateTableEnum, GenerateDieselTable, Selectable, Queryable, Debug, PartialEq)]
#[diesel(table_name = product_table)]
pub struct Product {
    pub id: i32,
    pub user_id: i32,
    pub paid: bool,
    pub production_date: NaiveDate,
    pub taken_date: NaiveDate,
    pub price: f64,
    pub amount: i32,
    pub description: String,
}

diesel::joinable!(product_table -> user_table(user_id));
diesel::allow_tables_to_appear_in_same_query!(product_table, user_table);

impl Migrationable for Product {
    fn get_up_migration(builder: impl SchemaBuilder) -> String {
        Table::create()
            .table(ProductTable::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(ProductTable::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new(ProductTable::UserId).integer())
            .col(ColumnDef::new(ProductTable::Paid).boolean())
            .col(ColumnDef::new(ProductTable::ProductionDate).date_time())
            .col(ColumnDef::new(ProductTable::TakenDate).date_time())
            .col(ColumnDef::new(ProductTable::Price).double())
            .col(ColumnDef::new(ProductTable::Amount).integer())
            .col(ColumnDef::new(ProductTable::Description).text())
            .foreign_key(
                ForeignKey::create()
                    .name("FK_ProductToUser")
                    .from(ProductTable::Table, ProductTable::UserId)
                    .to(UserTable::Table, UserTable::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .build(builder)
    }

    fn get_down_migration(builder: impl SchemaBuilder) -> String {
        Table::drop()
            .table(ProductTable::Table)
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
        let product_sqlite = Product::get_up_migration(SqliteQueryBuilder);
        let product_postgresql = Product::get_up_migration(PostgresQueryBuilder);

        println!("Sqlite -> \n{product_sqlite}");
        println!("Postgresql -> \n{product_postgresql}");
    }

    #[test]
    fn test_insert_product() {
        let conn = &mut create_connection();
        let product = ProductNoId {
            user_id: 2,
            paid: false,
            production_date: NaiveDate::from_ymd_opt(2025, 2, 3).unwrap(),
            taken_date: NaiveDate::from_ymd_opt(2025, 2, 3).unwrap(),
            price: 11000.0,
            amount: 2,
            description: String::from("taken was happen!!"),
        };

        let _result = diesel::insert_into(super::product_table::table)
            .values(&product)
            .execute(conn)
            .expect("error when inserting data");
    }
}
