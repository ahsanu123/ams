use chrono::NaiveDate;
use custom_macro::GenerateTableEnum;
use sea_query::{ColumnDef, Iden, Table, TableCreateStatement};

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
