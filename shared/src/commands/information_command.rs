use crate::helper::ENV_VAR;

pub async fn get_connection_string() -> String {
    ENV_VAR.sqlite_connection_string.clone()
}
