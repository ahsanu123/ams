pub use sea_orm_migration::prelude::*;

mod migration1;

pub struct MainMigrator;

#[async_trait::async_trait]
impl MigratorTrait for MainMigrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            // migration1
            Box::new(migration1::m202509200753_create_customer_table::Migration),
            Box::new(migration1::m202602171000_create_prices_table::Migration),
            Box::new(migration1::m202602171046_create_balance_table::Migration),
            Box::new(migration1::m202602171053_create_billing_table::Migration),
            Box::new(migration1::m202602171404_create_retrieve_data_table::Migration),
            Box::new(migration1::m202602171405_create_billing_retrieve_data_table::Migration),
            Box::new(migration1::m202602171412_create_balance_billing_table::Migration),
            Box::new(migration1::m202602171414_create_data_record_table::Migration),
            Box::new(migration1::m202602171438_seeding_customer::Migration),
            Box::new(migration1::m202602171440_seeding_price::Migration),
        ]
    }
}
