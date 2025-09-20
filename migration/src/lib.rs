pub use sea_orm_migration::prelude::*;

mod migration1;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(migration1::m202509200753_create_user_table::Migration),
            Box::new(migration1::m202509200754_create_production_record_table::Migration),
            Box::new(migration1::m202509200804_create_taking_record_table::Migration),
            Box::new(migration1::m202509200811_create_price_history_table::Migration),
            Box::new(migration1::m202509200815_create_payment_history_table::Migration),
            Box::new(migration1::m202509200818_create_soybean_price_history_table::Migration),
            Box::new(migration1::m202509200820_create_money_history_table::Migration),
        ]
    }
}
