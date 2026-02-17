use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let customer_id_fk = &mut ForeignKey::create()
            .from_tbl("billing")
            .from_col("customer_id")
            .to_tbl("customers")
            .to_col("customer_id")
            .on_update(ForeignKeyAction::Cascade)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table("billing")
                    .if_not_exists()
                    .col(big_pk_auto("billing_id"))
                    .col(big_integer("customer_id"))
                    .col(date_time("date"))
                    .foreign_key(customer_id_fk)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("billing").if_exists().to_owned())
            .await
    }
}
