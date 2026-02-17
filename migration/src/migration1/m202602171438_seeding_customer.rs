use std::vec;

use chrono::{Local, NaiveDateTime};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[enum_def]
struct Customer {
    customer_id: i64,
    customer_name: String,
    is_active: bool,
    is_admin: bool,
    created_date: NaiveDateTime,
    updated_date: NaiveDateTime,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let select_query = &mut Query::select()
            .columns([
                CustomerIden::CustomerName,
                CustomerIden::IsActive,
                CustomerIden::IsAdmin,
                CustomerIden::CreatedDate,
                CustomerIden::UpdatedDate,
            ])
            .from(CustomerIden::Table)
            .to_owned();

        let select_result = db.query_all(select_query).await.unwrap();

        println!("{:?}", select_result);

        let predefined_customer: Vec<String> = vec![
            "tresno".into(),
            "lurah".into(),
            "sinin".into(),
            "misbah".into(),
        ];
        let now = Local::now().naive_local();

        let query = &mut Query::insert()
            .into_table(CustomerIden::Table)
            .columns([
                CustomerIden::CustomerName,
                CustomerIden::IsAdmin,
                CustomerIden::IsActive,
                CustomerIden::CreatedDate,
                CustomerIden::UpdatedDate,
            ])
            .to_owned();

        for name in predefined_customer {
            query.values_panic([
                name.into(),
                false.into(),
                true.into(),
                now.into(),
                now.into(),
            ]);
        }
        db.execute(query)
            .await
            .expect("err when seeding customer!!");

        // NOTE:
        // - query all
        // - insert zero balance with customer_id

        // .table("balance")
        // .if_not_exists()
        // .col(big_pk_auto("balance_id"))
        // .col(big_integer("customer_id"))
        // .col(big_integer("value"))
        // .col(date_time("date").not_null())
        // .col(integer("transaction_type")) // map this to enum

        Ok(())
    }
}

#[cfg(test)]
mod seeding_customer_test {
    use super::*;

    #[test]
    fn check_generated_inden() {
        assert_eq!(CustomerIden::Table.to_string(), "customer");
        assert_eq!(CustomerIden::IsAdmin.to_string(), "is_admin");
    }

    #[test]
    fn generated_customer_insert_seed() {
        let predefined_customer: Vec<String> = vec![
            "tresno".into(),
            "lurah".into(),
            "sinin".into(),
            "misbah".into(),
        ];
        let now = Local::now().naive_local();

        let query = &mut Query::insert()
            .into_table(CustomerIden::Table)
            .columns([
                CustomerIden::CustomerName,
                CustomerIden::IsAdmin,
                CustomerIden::IsActive,
                CustomerIden::CreatedDate,
                CustomerIden::UpdatedDate,
            ])
            .to_owned();

        for name in predefined_customer {
            query.values_panic([
                name.into(),
                false.into(),
                true.into(),
                now.into(),
                now.into(),
            ]);
        }

        println!("{}", query.to_string(SqliteQueryBuilder));
    }
}
