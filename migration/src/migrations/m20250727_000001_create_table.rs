use crate::models::*;
use sea_orm_migration::{async_trait::async_trait, prelude::*, schema::*, MigrationTrait};

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(payment_record::PaymentRecord)]
    }
}
