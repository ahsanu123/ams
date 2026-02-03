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

// #[cfg(test)]
// mod test {
//     use chrono::Local;
//
//     use crate::repositories::abstract_repository_trait::AbstractRepository;
//
//     use super::*;
//
//     #[tokio::test]
//     async fn repo_create_user() {
//         let new_user = user_table::ActiveModel {
//             id: NotSet,
//             username: Set("Ahsanu".into()),
//             is_active: Set(true),
//             is_admin: Set(false),
//             money: Set(100000),
//             created_date: Set(Local::now().naive_local()),
//             updated_date: Set(Local::now().naive_local()),
//         };
//
//         let mut user_repo = UserRepository::default();
//
//         let result = user_repo.user_table.create(new_user).await;
//
//         println!("result {result:?}");
//     }
// }
