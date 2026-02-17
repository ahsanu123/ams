use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let balance_id_fk = &mut ForeignKey::create()
            .from_tbl("balance_billing")
            .from_col("balance_id")
            .to_tbl("balance")
            .to_col("balance_id")
            .on_update(ForeignKeyAction::Cascade)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        let billing_id_fk = &mut ForeignKey::create()
            .from_tbl("balance_billing")
            .from_col("billing_id")
            .to_tbl("billing")
            .to_col("billing_id")
            .on_update(ForeignKeyAction::Cascade)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table("balance_billing")
                    .if_not_exists()
                    .col(big_pk_auto("balance_billing_id"))
                    .col(big_integer("balance_id"))
                    .col(big_integer("billing_id"))
                    .foreign_key(balance_id_fk)
                    .foreign_key(billing_id_fk)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table("balance_billing")
                    .if_exists()
                    .to_owned(),
            )
            .await
    }
}
