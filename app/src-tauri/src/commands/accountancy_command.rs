use crate::model::user::User;
use chrono::NaiveDate;
use tauri;

#[tauri::command]
pub async fn accoutancy_command_get_total_bill_for_user(
    user: User,
    from: NaiveDate,
    to: NaiveDate,
) -> Result<f64, String> {
    todo!()
}
