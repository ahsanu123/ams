use crate::helper::ENV_VAR;
use ams_entity::prelude::*;
use sea_orm::{sqlx::Connection, DatabaseConnection, EntityTrait};

pub trait AbstractRepository {}

pub trait GetSqlConnectionTrait {
    fn get_connection() -> DatabaseConnection;
}

pub fn get_by_id(id: i32) {
    // User::find_by_id(id)
}

pub fn db_conn() {
    // let mut connection_option = ConnectionOption
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::helper::ENV_VAR;
    use sea_orm::{Database, DatabaseConnection, EntityTrait};

    #[tokio::test]
    async fn create_sea_orm_connection() {
        let db: DatabaseConnection = Database::connect(ENV_VAR.sqlite_connection_string.clone())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn abstract_repo_get_by_id() {
        let db: DatabaseConnection = Database::connect(ENV_VAR.sqlite_connection_string.clone())
            .await
            .unwrap();

        let may_user = UserTable::find_by_id(1).one(&db).await.unwrap();

        if let Some(user) = may_user {
            println!("Found some user");
        } else {
            println!("cant find user");
        }
    }
}
