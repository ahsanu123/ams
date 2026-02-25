use crate::repositories::{
    abstract_repository_trait::AbstractRepository, database_connection::get_database_connection,
};
use ams_entity::{prelude::*, user_table};
use chrono::Local;
use sea_orm::{ActiveValue::NotSet, ActiveValue::Set, EntityTrait, QueryFilter, entity::*};

pub trait UserManagementCommandTrait {
    async fn create_new_user(&mut self, username: String) -> i32;
    async fn insert_new_user(&mut self, new_user: user_table::Model) -> i32;
    async fn get_all_user(&mut self) -> Vec<user_table::Model>;
    async fn get_all_active_user(&mut self) -> Vec<user_table::Model>;
    async fn upsert_user(&mut self, user: user_table::Model) -> i32;
    async fn get_by_user_id(&mut self, id: i32) -> Option<user_table::Model>;
}

pub struct UserManagementCommand {
    user_table: UserTable,
}

impl Default for UserManagementCommand {
    fn default() -> Self {
        Self {
            user_table: UserTable,
        }
    }
}

impl UserManagementCommandTrait for UserManagementCommand {
    async fn create_new_user(&mut self, username: String) -> i32 {
        let conn = get_database_connection().await;

        let user_exist = UserTable::find()
            .filter(user_table::Column::Username.eq(username.clone()))
            .one(conn)
            .await
            .unwrap();

        if user_exist.is_some() {
            return 0;
        }

        let active_model = user_table::ActiveModel {
            id: NotSet,
            username: Set(username),
            is_active: Set(true),
            is_admin: Set(false),
            money: Set(0),
            created_date: Set(Local::now().naive_local()),
            updated_date: Set(Local::now().naive_local()),
        };

        let result = self.user_table.create(active_model).await.unwrap();

        result.id
    }

    async fn insert_new_user(&mut self, new_user: user_table::Model) -> i32 {
        let conn = get_database_connection().await;

        let username = new_user.clone().username;

        let user_exist = UserTable::find()
            .filter(user_table::Column::Username.eq(username))
            .one(conn)
            .await
            .unwrap();

        if user_exist.is_some() {
            return 0;
        }

        let active_model = user_table::ActiveModel {
            id: NotSet,
            username: Set(new_user.username),
            is_active: Set(new_user.is_active),
            is_admin: Set(new_user.is_admin),
            money: Set(new_user.money),
            created_date: Set(new_user.created_date),
            updated_date: Set(new_user.updated_date),
        };

        let result = self.user_table.create(active_model).await.unwrap();

        result.id
    }

    async fn get_all_user(&mut self) -> Vec<user_table::Model> {
        self.user_table.get_all().await.unwrap()
    }

    async fn get_all_active_user(&mut self) -> Vec<user_table::Model> {
        let conn = get_database_connection().await;

        UserTable::find()
            .filter(user_table::Column::IsActive.eq(true))
            .all(conn)
            .await
            .unwrap()
    }

    async fn upsert_user(&mut self, user: user_table::Model) -> i32 {
        let active_model = user_table::ActiveModel {
            id: Set(user.id),
            username: Set(user.username),
            is_active: Set(user.is_active),
            is_admin: Set(user.is_admin),
            money: Set(user.money),
            created_date: NotSet,
            updated_date: Set(Local::now().naive_local()),
        };

        let result = self.user_table.update_by_model(active_model).await.unwrap();

        result.id
    }

    async fn get_by_user_id(&mut self, id: i32) -> Option<user_table::Model> {
        self.user_table.get_by_id(id).await.unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn command_new_user() {
        let new_user = user_table::Model {
            id: 0,
            username: "Ahsanu".into(),
            is_active: true,
            is_admin: false,
            money: 0,
            created_date: Local::now().naive_local(),
            updated_date: Local::now().naive_local(),
        };

        let mut command = UserManagementCommand::default();
        let result = command.insert_new_user(new_user).await;
        println!("result: {result:#?}");
    }

    #[tokio::test]
    async fn command_get_all_user() {
        let mut command = UserManagementCommand::default();
        let result = command.get_all_user().await;

        println!("result: {result:#?}");
    }
}
