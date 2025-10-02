pub use sea_orm_migration::prelude::*;

mod migration1;
mod migration2;
pub mod seeds;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            // migration1
            Box::new(migration1::m202509200753_create_user_table::Migration),
            Box::new(migration1::m202509200811_create_price_history_table::Migration),
            Box::new(migration1::m202509200818_create_soybean_price_history_table::Migration),
            Box::new(migration1::m202509200754_create_production_record_table::Migration),
            Box::new(migration1::m202509200804_create_taking_record_table::Migration),
            Box::new(migration1::m202509200815_create_payment_history_table::Migration),
            Box::new(migration1::m202509200820_create_money_history_table::Migration),
            Box::new(migration1::m202509211820_create_password_hash_table::Migration),
            //
            // migration2
            // Box::new(migration2::m202509200832_create_foreign_key_taking_record::Migration),
            // Box::new(migration2::m202509200903_create_foreign_key_production_record::Migration),
            // Box::new(migration2::m202509200843_create_foreign_key_payment_history::Migration),
            // Box::new(migration2::m202509200909_create_foreign_key_money_history::Migration),
            //
            // seeds
            Box::new(seeds::price_history_seed::Migration),
            Box::new(seeds::user_seeds::Migration),
        ]
    }
}
