use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let customer_id_fk = &mut ForeignKey::create()
            .from_tbl("retrieve_data")
            .from_col("customer_id")
            .to_tbl("customers")
            .to_col("customer_id")
            .on_update(ForeignKeyAction::Cascade)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        let price_id_fk = &mut ForeignKey::create()
            .from_tbl("retrieve_data")
            .from_col("price_id")
            .to_tbl("price")
            .to_col("price_id")
            .on_update(ForeignKeyAction::Cascade)
            .on_delete(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .create_table(
                Table::create()
                    .table("retrieve_data")
                    .if_not_exists()
                    .col(big_pk_auto("retrieve_data_id"))
                    .col(big_integer("customer_id"))
                    .col(big_integer("price_id"))
                    .col(integer("amount"))
                    .col(date_time("date"))
                    .col(boolean("is_paid"))
                    .foreign_key(customer_id_fk)
                    .foreign_key(price_id_fk)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("retrieve_data").if_exists().to_owned())
            .await
    }
}
