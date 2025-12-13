use crate::repositories::{
    abstract_repository_trait::AbstractRepository, get_sql_connection_trait::GetSqlConnectionTrait,
};
use ams_entity::{prelude::*, price_history_table};
use chrono::Local;
use sea_orm::{
    ActiveValue::{NotSet, Set},
    EntityTrait, QueryOrder,
    prelude::async_trait,
};
use std::fmt::Error;

#[async_trait::async_trait]
pub trait DregPriceCommandTrait {
    async fn get_latest_dreg_price() -> Result<price_history_table::Model, Error>;
    async fn update_dreg_price(new_price: i64) -> price_history_table::Model;
    async fn get_all_dreg_price() -> Vec<price_history_table::Model>;
}

pub struct DregPriceCommand;

#[async_trait::async_trait]
impl DregPriceCommandTrait for DregPriceCommand {
    async fn get_latest_dreg_price() -> Result<price_history_table::Model, Error> {
        let conn = PriceHistoryTable::get_connection().await;

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

    async fn update_dreg_price(new_price: i64) -> price_history_table::Model {
        let updated_data = price_history_table::ActiveModel {
            id: NotSet,
            created_date: Set(Local::now().naive_local()),
            price: Set(new_price),
        };

        PriceHistoryTable::create(updated_data).await.unwrap()
    }

    async fn get_all_dreg_price() -> Vec<price_history_table::Model> {
        PriceHistoryTable::get_all().await.unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn command_get_latest_dreg_price() {
        let result = DregPriceCommand::get_latest_dreg_price().await;

        println!("result: {result:#?}");
    }

    #[tokio::test]
    async fn command_update_dreg_price() {
        let result = DregPriceCommand::update_dreg_price(12000).await;

        println!("result: {result:#?}");
    }

    #[tokio::test]
    async fn command_get_all_dreg_price() {
        let result = DregPriceCommand::get_all_dreg_price().await;

        println!("result: {result:#?}");
    }
}
