use crate::helper;
use sqlx::{Pool, Sqlite, SqlitePool};

// TODO: add argument to choose what database to use

pub async fn create_connection() -> Pool<Sqlite> {
    // let env_var = helper::environment_variable::ENV_VAR.get().unwrap();

    // let pool = SqlitePool::connect(&env_var.sqlite_connection_string)
    let pool = SqlitePool::connect("sqlite:ams.db").await.unwrap();

    pool
}
