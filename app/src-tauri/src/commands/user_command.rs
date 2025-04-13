use crate::model::user::User;
use tauri;

#[tauri::command]
pub async fn login(user: User) -> Result<bool, String> {
    todo!()
}
#[tauri::command]
pub async fn logout(user: User) -> Result<bool, String> {
    todo!()
}

#[cfg(test)]
mod test {

    #[tokio::test]
    async fn todo() {
        todo!();
    }
}
