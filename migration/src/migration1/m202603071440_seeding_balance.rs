use chrono::{Local, NaiveDateTime};
use sea_orm_migration::{
    prelude::*,
    schema::*,
    sea_orm::{DatabaseBackend, FromQueryResult, Statement, TransactionTrait},
};

#[enum_def]
struct Balance {
    balance_id: i64,
    customer_id: i64,
    value: f64,
    changed_value: f64,
    date: NaiveDateTime,
    transaction_type: i32,
}

#[derive(FromQueryResult)]
struct Customer {
    customer_id: i64,
    customer_name: String,
    is_active: bool,
    is_admin: bool,
    created_date: NaiveDateTime,
    updated_date: NaiveDateTime,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let stmt = Statement::from_string(DatabaseBackend::Sqlite, "SELECT * FROM customer;");
        let customers = Customer::find_by_statement(stmt).all(db).await?;

        let now = Local::now().naive_local();

        let transaction = db.begin().await?;

        for c in customers {
            let query = &mut Query::insert()
                .into_table(BalanceIden::Table)
                .columns([
                    BalanceIden::CustomerId,
                    BalanceIden::Value,
                    BalanceIden::ChangedValue,
                    BalanceIden::Date,
                    BalanceIden::TransactionType,
                ])
                .values_panic([
                    c.customer_id.into(),
                    0.into(),
                    0.into(),
                    now.into(),
                    0.into(), // transaction_type = "TopUp"
                ])
                .to_owned();

            transaction
                .execute(query)
                .await
                .expect("err when seeding balance!!");
        }

        transaction.commit().await?;

        Ok(())
    }
}
