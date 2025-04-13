use chrono::NaiveDate;
use tauri;

use crate::model::user::User;

#[tauri::command]
pub async fn get_report_detail(user: User, from: NaiveDate, to: NaiveDate) -> Result<f64, String> {
    todo!()
}

#[cfg(test)]
mod test {

    #[tokio::test]
    async fn todo() {
        todo!();
    }
}
