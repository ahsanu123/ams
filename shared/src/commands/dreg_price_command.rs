use crate::repositories::{
    abstract_repository_trait::AbstractRepository, database_connection::get_database_connection,
};
use ams_entity::{prelude::*, price_history_table};
use chrono::Local;
use sea_orm::{
    ActiveValue::{NotSet, Set},
    EntityTrait, QueryOrder,
    prelude::async_trait,
};
use std::fmt::Error;

pub trait DregPriceCommandTrait {
    async fn get_latest_dreg_price(&mut self) -> Result<price_history_table::Model, Error>;
    async fn update_dreg_price(&mut self, new_price: i64) -> price_history_table::Model;
    async fn get_all_dreg_price(&mut self) -> Vec<price_history_table::Model>;
}

pub struct DregPriceCommand {
    price_history_table: PriceHistoryTable,
}

impl Default for DregPriceCommand {
    fn default() -> Self {
        Self {
            price_history_table: PriceHistoryTable,
        }
    }
}

impl DregPriceCommandTrait for DregPriceCommand {
    async fn get_latest_dreg_price(&mut self) -> Result<price_history_table::Model, Error> {
        let conn = get_database_connection().await;

        let latest_dreg_price = PriceHistoryTable::find()
            .order_by_desc(price_history_table::Column::CreatedDate)
            .one(conn)
            .await
            .unwrap();

        if latest_dreg_price.is_none() {
            return Err(Error);
        }

        Ok(latest_dreg_price.unwrap())
    }

    async fn update_dreg_price(&mut self, new_price: i64) -> price_history_table::Model {
        let updated_data = price_history_table::ActiveModel {
            id: NotSet,
            created_date: Set(Local::now().naive_local()),
            price: Set(new_price),
        };

        self.price_history_table.create(updated_data).await.unwrap()
    }

    async fn get_all_dreg_price(&mut self) -> Vec<price_history_table::Model> {
        self.price_history_table.get_all().await.unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn command_get_latest_dreg_price() {
        let mut dreg_price_command = DregPriceCommand::default();
        let result = dreg_price_command.get_latest_dreg_price().await;

        println!("result: {result:#?}");
    }

    #[tokio::test]
    async fn command_update_dreg_price() {
        let mut dreg_price_command = DregPriceCommand::default();
        let result = dreg_price_command.update_dreg_price(12000).await;

        println!("result: {result:#?}");
    }

    #[tokio::test]
    async fn command_get_all_dreg_price() {
        let mut dreg_price_command = DregPriceCommand::default();
        let result = dreg_price_command.get_all_dreg_price().await;

        println!("result: {result:#?}");
    }
}
