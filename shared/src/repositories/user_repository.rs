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

        let active_user = UserTable::find()
            .filter(user_table::Column::IsActive.eq(true))
            .all(conn)
            .await
            .unwrap();

        active_user
    }
}

