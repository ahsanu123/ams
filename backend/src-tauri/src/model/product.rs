use chrono::NaiveDate;
use custom_macro::{GenerateDieselTable, GenerateTableEnum};
use diesel::prelude::*;
use diesel::{prelude::Queryable, Selectable};
use sea_query::{
    ColumnDef, Iden, IntoValueTuple, SchemaBuilder, SqliteQueryBuilder, Table,
    TableCreateStatement, Value,
};

use crate::{
    migration::migration_trait::Migrationable,
    repository::crud_repository_trait::IntoValueAndColumnTrait,
};

// #[derive(GenerateTableEnum, GenerateDieselTable, Queryable, Selectable, Insertable, Default)]
// #[diesel(table_name = product)]
#[derive(GenerateTableEnum)]
pub struct Product {
    pub id: u32,
    pub user_id: u32,
    pub paid: bool,
    pub production_date: NaiveDate,
    pub taken_date: NaiveDate,
    pub price: f64,
    pub amount: u32,
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
    use sea_query::PostgresQueryBuilder;

    #[test]
    fn test_product_up_migration() {
        let productSqlite = Product::get_up_migration(SqliteQueryBuilder);
        let productPostgresql = Product::get_up_migration(PostgresQueryBuilder);

        println!("Sqlite -> \n{productSqlite}");
        println!("Postgresql -> \n{productPostgresql}");
    }

    #[test]
    fn test_insert_product() {}
}
