use crate::model::user::User;

#[tauri::command]
pub async fn product_command_take_product(user: User, amount: u32) -> Result<f64, String> {
    todo!()
}
