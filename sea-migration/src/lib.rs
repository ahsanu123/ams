pub use sea_orm_migration::prelude::*;

use crate::models::{
    payment_record, payment_taking, price_record, production_record, saving_record, taking_record,
    user,
};

mod entity;
mod migrations;
mod models;
mod traits;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(user::Migration),
            Box::new(price_record::Migration),
            Box::new(production_record::Migration),
            Box::new(taking_record::Migration),
            Box::new(payment_record::Migration),
            Box::new(saving_record::Migration),
            Box::new(payment_taking::Migration),
        ]
    }
}
