use crate::migration1::m202509200753_create_user_table::UserTable;
use crate::migration1::m202509200815_create_payment_history_table::PaymentHistoryTable;
use sea_orm_migration::prelude::*;

const PAYMENT_HISTORY_USER_ID_NAME: &str = "FK_payment_history_user_id";

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let payment_history_user_id = TableForeignKey::new()
            .name(PAYMENT_HISTORY_USER_ID_NAME)
            .from_tbl(PaymentHistoryTable::Table)
            .from_col(PaymentHistoryTable::UserId)
            .to_tbl(UserTable::Table)
            .to_col(UserTable::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .alter_table(
                Table::alter()
                    .table(PaymentHistoryTable::Table)
                    .add_foreign_key(&payment_history_user_id)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let taking_record_user_id_drop_statement = ForeignKey::drop()
            .name(PAYMENT_HISTORY_USER_ID_NAME)
            .table(PaymentHistoryTable::Table)
            .to_owned();

        manager
            .drop_foreign_key(taking_record_user_id_drop_statement)
            .await
    }
}
