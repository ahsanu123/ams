use ams_entity::{prelude::*, user_table};
use sea_orm::{entity::*, prelude::async_trait::async_trait, query::*};

use crate::repositories::get_sql_connection_trait::GetSqlConnectionTrait;

#[async_trait]
pub trait AdditionalUserTableMethodTrait {
    async fn get_all_active_user() -> Vec<user_table::Model>;
}

#[async_trait]
impl AdditionalUserTableMethodTrait for UserTable {
    async fn get_all_active_user() -> Vec<user_table::Model> {
        let conn = Self::get_connection().await;

        let active_user = UserTable::find().all(conn).await.unwrap();

        active_user
    }
}

#[cfg(test)]
mod test {
    use chrono::Local;

    use crate::repositories::abstract_repository_trait::AbstractRepository;

    use super::*;

    #[tokio::test]
    async fn repo_create_user() {
        let new_user = user_table::ActiveModel {
            id: NotSet,
            username: Set("Ahsanu".into()),
            is_active: Set(true),
            is_admin: Set(false),
            money: Set(100000),
            created_date: Set(Local::now().naive_local()),
            updated_date: Set(Local::now().naive_local()),
        };

        let result = UserTable::create(new_user).await;

        println!("result {result:?}");
    }
}
