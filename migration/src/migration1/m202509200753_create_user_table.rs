use ams_macro::GenerateTableEnum;
use chrono::{NaiveDate, NaiveDateTime};
use sea_orm_migration::prelude::*;

#[derive(GenerateTableEnum, Debug)]
#[warn(dead_code)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub is_active: bool,
    pub is_admin: bool,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserTable::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserTable::Id)
                            .big_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserTable::Username).text())
                    .col(ColumnDef::new(UserTable::IsActive).boolean())
                    .col(ColumnDef::new(UserTable::IsAdmin).boolean())
                    .col(ColumnDef::new(UserTable::CreatedDate).date_time())
                    .col(ColumnDef::new(UserTable::UpdatedDate).date_time())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserTable::Table).if_exists().to_owned())
            .await
    }
}
