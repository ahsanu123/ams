use crate::repositories::database_connection::get_database_connection;
use ams_entity::{prelude::*, user_table};
use sea_orm::entity::*;

pub trait AdditionalUserTableMethodTrait {
    async fn get_all_active_user(&mut self) -> Vec<user_table::Model>;
}

#[derive(Default)]
pub struct UserRepository {}

impl AdditionalUserTableMethodTrait for UserRepository {
    async fn get_all_active_user(&mut self) -> Vec<user_table::Model> {
        let conn = get_database_connection().await;

        UserTable::find().all(conn).await.unwrap()
    }
}
