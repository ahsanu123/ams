use chrono::NaiveDate;

use crate::model::{record::Record, user::User};

// return id of new product
#[tauri::command]
pub async fn take_product(user: User, amount: u32) -> Result<i32, String> {
    todo!()
}

// return id of new product
#[tauri::command]
pub async fn update_product(product: Record) -> Result<i32, String> {
    todo!()
}

// return deleted count
#[tauri::command]
pub async fn delete_product(product: Record) -> Result<i32, String> {
    todo!()
}

#[tauri::command]
pub async fn get_all_taken_product_for_year(year: i32) -> Result<Vec<Record>, String> {
    todo!()
}

#[tauri::command]
pub async fn get_all_taken_product_for_month(month: NaiveDate) -> Result<Vec<Record>, String> {
    todo!()
}

#[tauri::command]
pub async fn get_all_taken_user_product_for_year(year: i32) -> Result<Vec<Record>, String> {
    todo!()
}

#[tauri::command]
pub async fn get_all_taken_user_product_for_month(month: NaiveDate) -> Result<Vec<Record>, String> {
    todo!()
}

#[cfg(test)]
mod test {

    #[tokio::test]
    async fn todo() {
        todo!();
    }
}
