use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("customer")
                    .if_not_exists()
                    .col(big_pk_auto("customer_id"))
                    .col(string("customer_name"))
                    .col(boolean("is_active"))
                    .col(boolean("is_admin"))
                    .col(date_time("created_date"))
                    .col(date_time("updated_date"))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("customers").if_exists().to_owned())
            .await
    }
}
