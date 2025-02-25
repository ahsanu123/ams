use crate::migration::migration_trait::Migrationable;
use chrono::NaiveDate;
use custom_macro::{GenerateDieselTable, GenerateTableEnum};
use diesel::prelude::*;
use diesel::{prelude::Queryable, Selectable};
use sea_query::{ColumnDef, Iden, SchemaBuilder, SqliteQueryBuilder, Table};

#[derive(Insertable)]
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

#[derive(GenerateTableEnum, GenerateDieselTable, Insertable, Selectable)]
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
            .build(builder)
    }

    fn get_down_migration(builder: impl SchemaBuilder) -> String {
        Table::drop().table(ProductTable::Table).build(builder)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::helper::sql_connection_helper::create_connection;
    use sea_query::PostgresQueryBuilder;

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
