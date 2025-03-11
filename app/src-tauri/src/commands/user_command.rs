use crate::model::user::User;
use tauri;

#[tauri::command]
pub async fn user_command_login(user: User) -> Result<bool, String> {
    todo!()
}
