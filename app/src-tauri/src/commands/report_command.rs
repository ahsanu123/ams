use chrono::NaiveDate;
use tauri;

use crate::model::user::User;

#[tauri::command]
pub async fn report_command_get_report_detail(
    user: User,
    from: NaiveDate,
    to: NaiveDate,
) -> Result<f64, String> {
    todo!()
}
