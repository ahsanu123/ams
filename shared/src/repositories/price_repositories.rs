use crate::{
    models::price::Price,
    repositories::base_repository_trait::{BaseRepository, BaseRepositoryErr},
};

pub enum PriceRepositoryErr {
    FailToGetLatest,
}

pub struct PriceRepository;

impl PriceRepository {
    pub fn get_latest(&mut self) -> Result<Price, PriceRepositoryErr> {
        todo!()
    }
}

impl BaseRepository<Price> for PriceRepository {
    async fn create(&mut self, model: Price) -> Result<i64, BaseRepositoryErr> {
        todo!()
    }

    async fn read(&mut self, id: i64) -> Result<Option<Price>, BaseRepositoryErr> {
        todo!()
    }

    async fn update(&mut self, model: Price) -> Result<Price, BaseRepositoryErr> {
        todo!()
    }

    async fn delete(&mut self, id: i64) -> Result<u64, BaseRepositoryErr> {
        todo!()
    }
}

// use crate::repositories::database_connection::get_database_connection;
// use ams_entity::{prelude::*, price_history_table};
// use sea_orm::{entity::*, query::*};
//
// pub trait AdditionalPriceHistoryTableMethodTrait {
//     async fn get_latest_price(&mut self) -> price_history_table::Model;
// }
//
// #[derive(Default)]
// pub struct PriceRepository {}
//
// impl AdditionalPriceHistoryTableMethodTrait for PriceRepository {
//     async fn get_latest_price(&mut self) -> price_history_table::Model {
//         let conn = get_database_connection().await;
//
//         let latest_price = PriceHistoryTable::find()
//             .order_by_desc(price_history_table::Column::CreatedDate)
//             .one(conn)
//             .await
//             .unwrap()
//             .expect("price is empty, please seed price!!");
//
//         latest_price
//     }
// }
