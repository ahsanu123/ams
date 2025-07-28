use ams_macro::GenerateTableEnum;
use chrono::NaiveDate;
use sea_orm_migration::{async_trait::async_trait, prelude::*, schema::*};
use sea_query::Table;

use crate::models::user::UserTable;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "migrating taking record"
    }
}

#[allow(dead_code)]
#[derive(GenerateTableEnum)]
pub struct TakingRecord {
    pub id: i64,
    pub user_id: i64,
    pub amount: i32,
    pub date: NaiveDate,
    pub paid: bool,
}

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TakingRecordTable::Table)
                    .if_not_exists()
                    .col(pk_auto(TakingRecordTable::Id))
                    .col(integer(TakingRecordTable::Amount))
                    .col(date(TakingRecordTable::Date))
                    .col(boolean(TakingRecordTable::Paid))
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_taking-record_user-id")
                            .from(TakingRecordTable::Table, TakingRecordTable::UserId)
                            .to(UserTable::Table, UserTable::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TakingRecordTable::Table).to_owned())
            .await
    }
}
