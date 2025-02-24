use crate::{
    // helper::sql_helper::{SqlConnectionProvider, SqlxHelper},
    model::product::Product,
};
use sqlx;

use super::migration_trait::MigrationAble;

// pub fn migrate_up() {
//     let conn = SqlConnectionProvider::default();
//     let list_migration = vec![Product::get_up_migration()];
//
//     list_migration.iter().for_each(|table_expr| {
//         conn.migrate_table(&table_expr);
//     });
// }
//
// pub fn migration_down() {
//     todo!()
// }
