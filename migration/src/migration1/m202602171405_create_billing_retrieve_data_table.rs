use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let billing_id_fk = &mut ForeignKey::create()
            .from_tbl("billing_retrieve_data")
            .from_col("billing_id")
            .to_tbl("billing")
            .to_col("billing_id")
            .on_update(ForeignKeyAction::Cascade)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        let retrieve_data_id_fk = &mut ForeignKey::create()
            .from_tbl("billing_retrieve_data")
            .from_col("retrieve_data_id")
            .to_tbl("retrieve_data")
            .to_col("retrieve_data_id")
            .on_update(ForeignKeyAction::Cascade)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table("billing_retrieve_data")
                    .if_not_exists()
                    .col(big_pk_auto("billing_retrieve_data_id"))
                    .col(big_integer("billing_id"))
                    .col(big_integer("retrieve_data_id"))
                    .foreign_key(billing_id_fk)
                    .foreign_key(retrieve_data_id_fk)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table("billing_retrieve_data")
                    .if_exists()
                    .to_owned(),
            )
            .await
    }
}
