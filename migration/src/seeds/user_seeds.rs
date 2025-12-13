use ams_entity::{prelude::*, user_table};
use chrono::Local;
use sea_orm_migration::{
    prelude::*,
    sea_orm::{
        ActiveValue::{NotSet, Set},
        EntityTrait, QueryFilter,
        entity::*,
    },
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let predefined_users: Vec<user_table::ActiveModel> = vec![
            user_table::ActiveModel {
                id: NotSet,
                username: Set("lurah".into()),
                is_active: Set(true),
                is_admin: Set(false),
                money: Set(0),
                created_date: Set(Local::now().naive_local()),
                updated_date: Set(Local::now().naive_local()),
            },
            user_table::ActiveModel {
                id: NotSet,
                username: Set("tresno".into()),
                is_active: Set(true),
                is_admin: Set(false),
                money: Set(0),
                created_date: Set(Local::now().naive_local()),
                updated_date: Set(Local::now().naive_local()),
            },
            user_table::ActiveModel {
                id: NotSet,
                username: Set("sinin".into()),
                is_active: Set(true),
                is_admin: Set(false),
                money: Set(0),
                created_date: Set(Local::now().naive_local()),
                updated_date: Set(Local::now().naive_local()),
            },
            user_table::ActiveModel {
                id: NotSet,
                username: Set("misbah".into()),
                is_active: Set(true),
                is_admin: Set(false),
                money: Set(0),
                created_date: Set(Local::now().naive_local()),
                updated_date: Set(Local::now().naive_local()),
            },
        ];

        for user in predefined_users {
            let username = user.clone().username.unwrap();
            let user_exists = UserTable::find()
                .filter(user_table::Column::Username.eq(username))
                .one(db)
                .await
                .unwrap();

            if user_exists.is_none() {
                let _ = user.insert(db).await;
            }
        }

        Ok(())
    }
}
