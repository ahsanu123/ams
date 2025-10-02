use crate::repositories::abstract_repository_trait::AbstractRepository;
use crate::repositories::get_sql_connection_trait::GetSqlConnectionTrait;
use crate::repositories::user_repository::AdditionalUserTableMethodTrait;
use ams_entity::prelude::*;
use ams_entity::user_table;
use chrono::Local;
use sea_orm::ActiveValue::NotSet;
use sea_orm::ActiveValue::Set;
use sea_orm::QueryFilter;
use sea_orm::prelude::async_trait;
use sea_orm::{EntityTrait, entity::*};

#[async_trait::async_trait]
pub trait UserManagementCommandTrait {
    async fn insert_new_user(new_user: user_table::ActiveModel) -> i32;
    async fn get_all_user() -> Vec<user_table::Model>;
    async fn get_all_active_user() -> Vec<user_table::Model>;
    async fn upsert_user(user: user_table::Model) -> i32;
}

pub struct UserManagementCommand;

#[async_trait::async_trait]
impl UserManagementCommandTrait for UserManagementCommand {
    async fn insert_new_user(new_user: user_table::ActiveModel) -> i32 {
        let conn = UserTable::get_connection().await;
        let username = new_user.clone().username.unwrap();

        let user_exist = UserTable::find()
            .filter(user_table::Column::Username.eq(username))
            .one(conn)
            .await
            .unwrap();

        if user_exist.is_some() {
            return 0;
        }

        let result = UserTable::create(new_user).await.unwrap();

        result.id
    }

    async fn get_all_user() -> Vec<user_table::Model> {
        UserTable::get_all().await.unwrap()
    }

    async fn get_all_active_user() -> Vec<user_table::Model> {
        UserTable::get_all_active_user().await
    }

    async fn upsert_user(user: user_table::Model) -> i32 {
        let active_model = user_table::ActiveModel {
            id: NotSet,
            username: Set(user.username),
            is_active: Set(user.is_active),
            is_admin: Set(user.is_admin),
            money: Set(user.money),
            created_date: NotSet,
            updated_date: Set(Local::now().naive_local()),
        };

        let result = UserTable::update_by_model(active_model).await.unwrap();

        result.id
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[tokio::test]
    async fn command_new_user() {
        let new_user = user_table::ActiveModel {
            id: NotSet,
            username: Set("Melbourne".into()),
            is_active: Set(true),
            is_admin: Set(false),
            money: Set(0),
            created_date: Set(Local::now().naive_local()),
            updated_date: Set(Local::now().naive_local()),
        };

        let result = UserManagementCommand::insert_new_user(new_user).await;
        println!("result: {result:?}");
    }

    #[tokio::test]
    async fn command_get_all_user() {
        let result = UserManagementCommand::get_all_user().await;

        println!("result: {result:#?}");
    }
}
