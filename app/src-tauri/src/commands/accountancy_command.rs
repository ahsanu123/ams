use crate::model::{dreg_price_history::DregPrice, money_record::MoneyRecord, user::User};
use chrono::NaiveDate;
use tauri;

#[tauri::command]
pub async fn get_user_bill_for_year(user: User, year: i32) -> Result<Vec<MoneyRecord>, String> {
    todo!()
}

#[tauri::command]
pub async fn get_user_bill_for_month(
    user: User,
    month: NaiveDate,
) -> Result<Vec<MoneyRecord>, String> {
    todo!()
}

#[tauri::command]
pub async fn get_user_saving(user: User) -> Result<f64, String> {
    // get latest of user money history
    // then add it wit amount
    // and insert new data of it
    todo!()
}

#[tauri::command]
pub async fn add_user_saving(user: User, amount: f64) -> Result<f64, String> {
    // get latest of user money history
    // then add it wit amount
    // and insert new data of it
    todo!()
}

#[tauri::command]
pub async fn add_user_bill(user: User, amount: f64) -> Result<f64, String> {
    // get latest of user money history
    // then add it wit amount
    // and insert new data of it
    todo!()
}

#[tauri::command]
pub async fn get_latest_user_money_history(user: User) -> Result<MoneyRecord, String> {
    todo!()
}

#[tauri::command]
pub async fn get_current_dreg_price() -> Result<f64, String> {
    todo!()
}

#[tauri::command]
pub async fn add_dreg_price(new_price: f64) -> Result<f64, String> {
    todo!()
}

#[tauri::command]
pub async fn get_all_dreg_prices() -> Result<Vec<DregPrice>, String> {
    todo!()
}

#[cfg(test)]
mod test {

    #[tokio::test]
    async fn todo() {
        todo!();
    }
}
