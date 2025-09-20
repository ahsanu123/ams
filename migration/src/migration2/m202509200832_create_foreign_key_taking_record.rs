use crate::migration1::m202509200753_create_user_table::UserTable;
use crate::migration1::m202509200804_create_taking_record_table::TakingRecordTable;
use crate::migration1::m202509200811_create_price_history_table::PriceHistoryTable;
use sea_orm_migration::prelude::*;

const TAKING_RECORD_USER_ID_NAME: &str = "FK_taking_record_user_id";
const TAKING_RECORD_PRICE_ID_NAME: &str = "FK_taking_record_price_id";

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let taking_record_user_id = TableForeignKey::new()
            .name(TAKING_RECORD_USER_ID_NAME)
            .from_tbl(TakingRecordTable::Table)
            .from_col(TakingRecordTable::UserId)
            .to_tbl(UserTable::Table)
            .to_col(UserTable::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();

        let taking_record_price_id = TableForeignKey::new()
            .name(TAKING_RECORD_PRICE_ID_NAME)
            .from_tbl(TakingRecordTable::Table)
            .from_col(TakingRecordTable::PriceId)
            .to_tbl(PriceHistoryTable::Table)
            .to_col(PriceHistoryTable::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .alter_table(
                Table::alter()
                    .table(TakingRecordTable::Table)
                    .add_foreign_key(&taking_record_user_id)
                    .add_foreign_key(&taking_record_price_id)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let taking_record_user_id_drop_statement = ForeignKey::drop()
            .name(TAKING_RECORD_USER_ID_NAME)
            .table(TakingRecordTable::Table)
            .to_owned();

        let taking_record_price_id_drop_statement = ForeignKey::drop()
            .name(TAKING_RECORD_PRICE_ID_NAME)
            .table(TakingRecordTable::Table)
            .to_owned();

        manager
            .drop_foreign_key(taking_record_user_id_drop_statement)
            .await?;

        manager
            .drop_foreign_key(taking_record_price_id_drop_statement)
            .await?;

        Ok(())
    }
}
