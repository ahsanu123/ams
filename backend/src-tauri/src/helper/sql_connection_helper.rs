use crate::helper;
use sqlx::{Pool, Sqlite, SqlitePool};

use super::environment_variable::ENV_VAR;

// TODO: add argument to choose what database to use

pub async fn create_connection() -> Pool<Sqlite> {
    let connection_string = &ENV_VAR.sqlite_connection_string;
    let pool = SqlitePool::connect(&connection_string)
        .await
        .expect("connection cant established");

    pool
}

#[cfg(test)]
mod test {
    use super::create_connection;

    #[tokio::test]
    async fn test_create_connection() {
        let conn = create_connection().await;
        conn.close();
        println!("Success connecting to database");
    }
}
