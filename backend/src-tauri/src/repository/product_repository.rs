use crate::model::{
    product::{Product, ProductTable},
    summary_information::SummaryInformation,
    user::User,
};
use crate::repository::crud_repository_trait::CrudRepositoryTrait;
use chrono::NaiveDate;

pub trait ProductRepositoryTrait {
    async fn change_product_price(&self, new_price: f64);
    async fn add_product_for_user(&self, user: User, amount: u32);
    async fn set_paid_status(&self, user: User, from: NaiveDate, to: NaiveDate, status: bool);
    async fn get_list_ampas_for_user(&self, user: User, date: NaiveDate) -> Vec<Product>;
    async fn get_bill_for_user(&self, user: User, date: NaiveDate) -> f64;
    async fn get_product_price(&self) -> f64;
    async fn get_product_price_history(&self) -> Vec<f64>;
    async fn get_summary_for_user(
        &self,
        user: User,
        date: NaiveDate,
        from: NaiveDate,
        to: NaiveDate,
    ) -> Vec<SummaryInformation>;
}

pub struct ProductRepository {}

impl ProductRepositoryTrait for ProductRepository {
    async fn change_product_price(&self, new_price: f64) {
        todo!()
    }

    async fn add_product_for_user(&self, user: User, amount: u32) {
        todo!()
    }

    async fn set_paid_status(&self, user: User, from: NaiveDate, to: NaiveDate, status: bool) {
        todo!()
    }

    async fn get_list_ampas_for_user(&self, user: User, date: NaiveDate) -> Vec<Product> {
        todo!()
    }

    async fn get_bill_for_user(&self, user: User, date: NaiveDate) -> f64 {
        todo!()
    }

    async fn get_product_price(&self) -> f64 {
        todo!()
    }

    async fn get_product_price_history(&self) -> Vec<f64> {
        todo!()
    }

    async fn get_summary_for_user(
        &self,
        user: User,
        date: NaiveDate,
        from: NaiveDate,
        to: NaiveDate,
    ) -> Vec<SummaryInformation> {
        todo!()
    }
}

impl Default for ProductRepository {
    fn default() -> Self {
        Self {}
    }
}

impl CrudRepositoryTrait<Product> for ProductRepository {
    async fn create(&self, data: &Product) {}

    async fn read(&self, id: u32) {
        todo!()
    }

    async fn update(&self, data: &Product) {
        todo!()
    }

    async fn delete(&self, id: u32) {
        todo!()
    }
}

#[cfg(test)]
mod test {

    use sea_query::{ColumnDef, Query, SqliteQueryBuilder, Table};
    use sea_query_binder::SqlxBinder;
    use sqlx::{Pool, Sqlite, SqlitePool};

    use crate::helper::sql_connection_helper::create_connection;

    use super::*;

    async fn create_table(pool: &Pool<Sqlite>) {
        let table_sql = Table::create()
            .table(ProductTable::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(ProductTable::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new(ProductTable::UserId).integer())
            .col(ColumnDef::new(ProductTable::Paid).boolean())
            .col(ColumnDef::new(ProductTable::ProductionDate).date())
            .col(ColumnDef::new(ProductTable::TakenDate).date())
            .col(ColumnDef::new(ProductTable::Price).float())
            .col(ColumnDef::new(ProductTable::Amount).integer())
            .col(ColumnDef::new(ProductTable::Description).text())
            .build(SqliteQueryBuilder);

        let result = sqlx::query(&table_sql).execute(pool).await;
        println!("Create table character: {result:?}\n");
    }

    #[tokio::test]
    async fn product_create() {
        let pool = create_connection().await;
        create_table(&pool).await;

        let (sql, values) = Query::insert()
            .into_table(ProductTable::Table)
            .columns([
                ProductTable::UserId,
                ProductTable::Paid,
                ProductTable::ProductionDate,
                ProductTable::TakenDate,
                ProductTable::Price,
                ProductTable::Amount,
                ProductTable::Description,
            ])
            .values_panic([
                1.into(),
                false.into(),
                NaiveDate::from_ymd_opt(2025, 5, 1).unwrap().into(),
                NaiveDate::from_ymd_opt(2025, 5, 12).unwrap().into(),
                11000.into(),
                2.into(),
                "taking dregs from storage".into(),
            ])
            .build_sqlx(SqliteQueryBuilder);

        let row = sqlx::query_with(&sql, values).execute(&pool).await.unwrap();
    }

    #[test]
    fn product_update() {
        todo!()
    }

    #[test]
    fn product_get() {
        todo!()
    }

    #[test]
    fn product_delete() {
        todo!();
    }
}
