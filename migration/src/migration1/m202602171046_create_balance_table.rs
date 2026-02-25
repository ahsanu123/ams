use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let customer_id_fk = &mut ForeignKey::create()
            .from_tbl("balance")
            .from_col("customer_id")
            .to_tbl("customer")
            .to_col("customer_id")
            .on_update(ForeignKeyAction::Cascade)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table("balance")
                    .if_not_exists()
                    .col(big_pk_auto("balance_id"))
                    .col(big_integer("customer_id"))
                    .col(big_integer("value"))
                    .col(date_time("date").not_null())
                    .col(integer("transaction_type")) // map this to enum
                    .foreign_key(customer_id_fk)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("balance").if_exists().to_owned())
            .await
    }
}
