use chrono::NaiveDate;
use custom_macro::GenerateTableEnum;
use sea_query::{ColumnDef, Iden, IntoValueTuple, Table, TableCreateStatement, Value};

use crate::repository::crud_repository_trait::IntoValueAndColumnTrait;

// use crate::migration::migration_trait::MigrationAble;

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

impl IntoValueAndColumnTrait<ProductTable, Product> for Product {
    fn columns() -> Vec<ProductTable> {
        vec![
            ProductTable::UserId,
            ProductTable::Paid,
            ProductTable::ProductionDate,
            ProductTable::TakenDate,
            ProductTable::Price,
            ProductTable::Amount,
            ProductTable::Description,
        ]
    }
}

impl IntoIterator for Product {
    type Item = Value;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            self.user_id.into(),
            self.paid.into(),
            self.production_date.into(),
            self.taken_date.into(),
            self.price.into(),
            self.amount.into(),
            self.description.into(),
        ]
        .into_iter()
    }
}

//============================================
// let (sql, values) = Query::insert()
//     .into_table(ProductTable::Table)
//     .columns(Product::columns()) // Dynamically fetch columns
//     .values_panic(product.into_values())
// .build_sqlx(SqliteQueryBuilder);

// impl MigrationAble for Product {
//     fn get_up_migration() -> TableCreateStatement {
//         Table::create()
//             .table(ProductTable::Table)
//             .if_not_exists()
//             .col(
//                 ColumnDef::new(ProductTable::Id)
//                     .integer()
//                     .not_null()
//                     .auto_increment()
//                     .primary_key(),
//             )
//             .col(ColumnDef::new(ProductTable::UserId).integer())
//             .col(ColumnDef::new(ProductTable::Paid).boolean())
//             .col(ColumnDef::new(ProductTable::ProductionDate).date_time())
//             .col(ColumnDef::new(ProductTable::TakenDate).date_time())
//             .col(ColumnDef::new(ProductTable::Price).double())
//             .col(ColumnDef::new(ProductTable::Amount).integer())
//             .col(ColumnDef::new(ProductTable::Description).text())
//             .clone()
//     }
//
//     fn get_down_migration() -> bool {
//         todo!()
//     }
// }
