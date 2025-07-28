use ams_macro::GenerateTableEnum;
use chrono::NaiveDate;
use sea_orm_migration::{async_trait::async_trait, prelude::*, schema::*};
use sea_query::Table;

use crate::models::user::UserTable;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "migrating saving record"
    }
}

#[allow(dead_code)]
#[derive(GenerateTableEnum)]
pub struct SavingRecord {
    pub id: i64,
    pub user_id: i64,
    pub amount: f64,
    pub date: NaiveDate,
    pub description: String,
}

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SavingRecordTable::Table)
                    .if_not_exists()
                    .col(pk_auto(SavingRecordTable::Id))
                    .col(integer(SavingRecordTable::UserId))
                    .col(float(SavingRecordTable::Amount))
                    .col(date(SavingRecordTable::Date))
                    .col(string(SavingRecordTable::Description))
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("fk_saving-record_user-id")
                            .from(SavingRecordTable::Table, SavingRecordTable::UserId)
                            .to(UserTable::Table, UserTable::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SavingRecordTable::Table).to_owned())
            .await
    }
}
