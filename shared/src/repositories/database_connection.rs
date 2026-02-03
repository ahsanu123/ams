use async_once_cell::OnceCell;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::helper::ENV_VAR;

static MAX_CONNECTION: u32 = 100;
static MIN_CONNECTION: u32 = 3;

pub static DATABASE_CONNECTION: OnceCell<DatabaseConnection> = OnceCell::new();

pub async fn get_database_connection() -> &'static DatabaseConnection {
    DATABASE_CONNECTION
        .get_or_init(async {
            let may_env = ENV_VAR.get();
            if may_env.is_none() {
                panic!("ENV_VAR is empty");
            }

            let mut connect_option =
                ConnectOptions::new(may_env.unwrap().sqlite_connection_string.clone());

            connect_option
                .max_connections(MAX_CONNECTION)
                .min_connections(MIN_CONNECTION)
                .sqlx_logging(true);

            Database::connect(connect_option).await.unwrap()
        })
        .await
}
