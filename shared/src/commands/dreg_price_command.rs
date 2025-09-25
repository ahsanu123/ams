use std::fmt::Error;

use ams_entity::{prelude::*, price_history_table};
use chrono::Local;
use sea_orm::{
    ActiveValue::{NotSet, Set},
    EntityTrait, QueryOrder,
};

use crate::repositories::{
    abstract_repository_trait::AbstractRepository, get_sql_connection_trait::GetSqlConnectionTrait,
};

pub async fn get_latest_dreg_price() -> Result<price_history_table::Model, Error> {
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

pub async fn update_dreg_price(new_price: i64) -> price_history_table::Model {
    let updated_data = price_history_table::ActiveModel {
        id: NotSet,
        created_date: Set(Local::now().naive_local()),
        price: Set(new_price),
    };

    let updated_result = PriceHistoryTable::create(updated_data).await.unwrap();

    updated_result
}

pub async fn get_all_dreg_price() -> Vec<price_history_table::Model> {
    let prices = PriceHistoryTable::get_all().await.unwrap();

    prices
}

#[cfg(test)]
mod test {
    #[test]
    fn name() {
        todo!();
    }
}
