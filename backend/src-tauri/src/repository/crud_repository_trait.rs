pub trait CrudRepositoryTrait<T> {
    async fn create(&self, data: &T);
    async fn read(&self, id: u32);
    async fn update(&self, data: &T);
    async fn delete(&self, id: u32);
}

pub trait IntoValueAndColumnTrait<T> {
    fn into_value(&self) -> Vec<sea_query::value>;
    fn columns() -> Vec<T>;
}

//============================================
// impl Product {
//     fn into_values(self) -> Vec<sea_query::Value> {
//         vec![
//             self.user_id.into(),
//             self.paid.into(),
//             self.production_date.into(),
//             self.taken_date.into(),
//             self.price.into(),
//             self.amount.into(),
//             self.description.into(),
//         ]
//     }
// }
//============================================
// impl Product {
//     fn columns() -> Vec<ProductTable> {
//         vec![
//             ProductTable::UserId,
//             ProductTable::Paid,
//             ProductTable::ProductionDate,
//             ProductTable::TakenDate,
//             ProductTable::Price,
//             ProductTable::Amount,
//             ProductTable::Description,
//         ]
//     }
// }
//============================================
// let (sql, values) = Query::insert()
//     .into_table(ProductTable::Table)
//     .columns(Product::columns()) // Dynamically fetch columns
//     .values_panic(product.into_values())
// .build_sqlx(SqliteQueryBuilder);
