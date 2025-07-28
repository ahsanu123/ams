use ams_macro::GenerateTableEnum;
use chrono::NaiveDate;
use sea_orm_migration::{async_trait::async_trait, prelude::*, schema::*};
use sea_query::Table;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "migrating user"
    }
}

#[allow(dead_code)]
#[derive(GenerateTableEnum)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub created_date: NaiveDate,
    pub password_hash: String,
}

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserTable::Table)
                    .if_not_exists()
                    .col(pk_auto(UserTable::Id))
                    .col(string(UserTable::Username))
                    .col(date(UserTable::CreatedDate))
                    .col(string(UserTable::PasswordHash))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserTable::Table).to_owned())
            .await
    }
}
