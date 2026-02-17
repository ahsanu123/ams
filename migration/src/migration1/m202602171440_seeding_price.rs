use chrono::{Local, NaiveDateTime};
use sea_orm_migration::{prelude::*, schema::*};

#[enum_def]
struct Price {
    price_id: i64,
    date: NaiveDateTime,
    value: f64,
}
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let now = Local::now().naive_local();
        let query = &mut Query::insert()
            .into_table(PriceIden::Table)
            .columns([PriceIden::Date, PriceIden::Value])
            .values_panic([now.into(), 11000.0.into()])
            .to_owned();

        db.execute(query).await.expect("err when seeding price!!");

        Ok(())
    }
}

#[cfg(test)]
mod seeding_customer_test {
    use super::*;

    #[test]
    fn generated_price_insert_seed() {
        let now = Local::now().naive_local();
        let query = &mut Query::insert()
            .into_table(PriceIden::Table)
            .columns([PriceIden::Date, PriceIden::Value])
            .values_panic([now.into(), 11000.0.into()])
            .to_owned();

        println!("{}", query.to_string(SqliteQueryBuilder));
    }
}
