use crate::migration1::m202509200753_create_user_table::UserTable;
use crate::migration1::m202509200820_create_money_history_table::MoneyHistoryTable;
use sea_orm_migration::prelude::*;

const MONEY_HISTORY_USER_ID_NAME: &str = "FK_money_history_user_id";

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let money_history_user_id = TableForeignKey::new()
            .name(MONEY_HISTORY_USER_ID_NAME)
            .from_tbl(MoneyHistoryTable::Table)
            .from_col(MoneyHistoryTable::UserId)
            .to_tbl(UserTable::Table)
            .to_col(UserTable::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .alter_table(
                Table::alter()
                    .table(MoneyHistoryTable::Table)
                    .add_foreign_key(&money_history_user_id)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let money_history_user_id_drop_statement = ForeignKey::drop()
            .name(MONEY_HISTORY_USER_ID_NAME)
            .table(MoneyHistoryTable::Table)
            .to_owned();

        manager
            .drop_foreign_key(money_history_user_id_drop_statement)
            .await
    }
}
