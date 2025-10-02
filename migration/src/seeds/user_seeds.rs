use ams_entity::{prelude::*, price_history_table, user_table};
use chrono::Local;
use sea_orm_migration::{
    prelude::*,
    sea_orm::{
        ActiveValue::{NotSet, Set},
        EntityTrait,
        entity::*,
        query::*,
    },
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let default_user = UserTable::find_by_id(1).one(db).await.unwrap();

        if !default_user.is_some() {
            let _ = user_table::ActiveModel {
                id: NotSet,
                username: Set("Ahsanu".into()),
                is_active: Set(true),
                is_admin: Set(false),
                money: Set(100000),
                created_date: Set(Local::now().naive_local()),
                updated_date: Set(Local::now().naive_local()),
            }
            .insert(db)
            .await;
        }

        Ok(())
    }
}
