use super::m202509200753_create_user_table::UserTable;
use ams_macro::GenerateTableEnum;
use sea_orm_migration::prelude::*;

const PASSWORD_HASH_USER_ID_NAME: &str = "FK_password_hash_user_id";

#[derive(GenerateTableEnum, Debug)]
#[warn(dead_code)]
pub struct HashPassword {
    pub id: u32,
    pub user_id: u32,
    pub hash: String,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let password_hash_user_id = &mut ForeignKey::create()
            .name(PASSWORD_HASH_USER_ID_NAME)
            .from_tbl(HashPasswordTable::Table)
            .from_col(HashPasswordTable::UserId)
            .to_tbl(UserTable::Table)
            .to_col(UserTable::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();
        manager
            .create_table(
                Table::create()
                    .table(HashPasswordTable::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(HashPasswordTable::Id)
                            .big_unsigned()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .foreign_key(password_hash_user_id)
                    .col(ColumnDef::new(HashPasswordTable::Hash).text().not_null())
                    .col(
                        ColumnDef::new(HashPasswordTable::UserId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(HashPasswordTable::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await
    }
}
