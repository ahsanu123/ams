use crate::helper::sql_connection_helper::create_connection;
use crate::model::product::{self, product_table, ProductNoId};
use crate::model::{
    product::{Product, ProductTable},
    summary_information::SummaryInformation,
    user::User,
};
use crate::repository::crud_repository_trait::CrudRepositoryTrait;
use chrono::NaiveDate;
use diesel::{AsChangeset, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use std::vec;

pub struct ProductRepository {}

impl CrudRepositoryTrait<Product> for ProductRepository {
    fn getAll(&self) -> Result<Vec<Product>, String> {
        let conn = &mut create_connection();
        let product_list = product_table::table.load(conn);

        match product_list {
            Ok(products) => Ok(products),
            Err(_) => Err(format!(
                "fail to load list product, {}, {}",
                file!(),
                line!()
            )),
        }
    }

    fn create(&self, data: &Product) -> Result<usize, String> {
        let conn = &mut create_connection();
        let insert_data: ProductNoId = data.into();
        let result = diesel::insert_into(product_table::table)
            .values(insert_data)
            .execute(conn);

        match result {
            Ok(inserted_row) => Ok(inserted_row),
            Err(_) => Err(format!("fail to insert product, {}, {}", file!(), line!())),
        }
    }

    fn read(&self, id: u32) -> Result<Product, String> {
        let conn = &mut create_connection();
        let result = product_table::table
            .filter(product_table::id.eq(id as i32))
            .first::<Product>(conn);

        match result {
            Ok(product) => Ok(product),
            Err(_) => Err(format!(
                "fail to get product with id {id}, {}, {}",
                file!(),
                line!()
            )),
        }
    }

    fn update(&self, data: &Product) -> Result<usize, String> {
        let conn = &mut create_connection();
        let update_data: ProductNoId = data.into();
        let update_query = product_table::table.filter(product_table::id.eq(data.id));

        let result = diesel::update(update_query).set(update_data).execute(conn);

        match result {
            Ok(updated_row) => Ok(updated_row),
            Err(_) => Err(format!("fail to update product, {}, {}", file!(), line!())),
        }
    }

    fn delete(&self, id: u32) -> Result<usize, String> {
        let conn = &mut create_connection();
        let delete_query = product_table::table.filter(product_table::id.eq(id as i32));

        let result = diesel::delete(delete_query).execute(conn);
        match result {
            Ok(deleted_row) => Ok(deleted_row),
            Err(_) => Err(format!("fail to delete product, {}, {}", file!(), line!())),
        }
    }
}

impl Default for ProductRepository {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn product_repository_create_product() {
        let product_repo = ProductRepository::default();
        let new_product = Product {
            id: 0, // just random id (it will removed)
            user_id: 1,
            paid: false,
            production_date: NaiveDate::from_ymd_opt(2025, 2, 3).unwrap(),
            taken_date: NaiveDate::from_ymd_opt(2025, 2, 3).unwrap(),
            price: 11000.0,
            amount: 1,
            description: "new inserted data".into(),
        };

        let inserted_row = product_repo
            .create(&new_product)
            .expect("cant insert new product!!");

        let products_list = product_repo.getAll().expect("cant get list of product");
        println!("success to insert {inserted_row} product, current table {products_list:#?}");
    }

    #[test]
    fn product_repository_read_product() {
        let product_repo = ProductRepository::default();
        let product_with_id_two = product_repo
            .read(2)
            .expect("product with id 2 is not found");

        println!("here is product with id 2 {product_with_id_two:#?}");
    }

    #[test]
    fn product_repository_update_product() {
        let product_repo = ProductRepository::default();
        let old_product_data = Product {
            id: 2, // just random id (it will removed)
            user_id: 1,
            paid: false,
            production_date: NaiveDate::from_ymd_opt(2025, 2, 3).unwrap(),
            taken_date: NaiveDate::from_ymd_opt(2025, 2, 3).unwrap(),
            price: 11000.0,
            amount: 1,
            description: "its old data, will updated soon!!!".into(),
        };

        let new_product_data = Product {
            id: 2, // just random id (it will removed)
            user_id: 1,
            paid: true,
            production_date: NaiveDate::from_ymd_opt(2025, 2, 3).unwrap(),
            taken_date: NaiveDate::from_ymd_opt(2025, 2, 3).unwrap(),
            price: 22000.0,
            amount: 2,
            description: "its updated data".into(),
        };

        //update to new data
        let result = product_repo.update(&new_product_data).unwrap();
        let updated_to_new_data = product_repo.read(new_product_data.id as u32).unwrap();

        assert_eq!(
            updated_to_new_data, new_product_data,
            "fail when trying to update into new product data"
        );

        // get back old data
        let result = product_repo.update(&old_product_data).unwrap();
        let updated_to_new_data = product_repo.read(new_product_data.id as u32).unwrap();

        assert_eq!(
            updated_to_new_data, old_product_data,
            "fail when trying to get back old product data!!!"
        );
    }

    #[test]
    fn product_repository_delete_product() {
        let product_repo = ProductRepository::default();

        let result = product_repo.delete(3); // trying to delete id 3 (its hardcoded)
        match result {
            Ok(_) => {
                println!("Success deleting product with id 3");
            }
            Err(_) => {
                println!("Failed to deleting product with id 3, make sure database has data, so its can deleted!!!!");
            }
        }
    }

    #[test]
    fn product_repository_get_all_product() {
        let product_repo = ProductRepository::default();

        let products_list = product_repo.getAll().expect("cant get list of product");
        println!("list of product {products_list:#?}");
    }
}
