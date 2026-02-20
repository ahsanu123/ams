use crate::{
    models::{price::Price, to_active_without_id_trait::ToActiveModel},
    repositories::{
        base_repository_trait::{BaseRepository, BaseRepositoryErr},
        database_connection::get_database_connection,
        generic_crud_repository::GenericCrudRepository,
    },
};
use ams_entity::prelude::Price as PriceDb;
use ams_entity::price as price_db;
use sea_orm::{prelude::*, query::*};

pub enum PriceRepositoryErr {
    FailToGetLatest,
}

pub struct PriceRepository;

impl PriceRepository {
    pub async fn get_latest(&mut self) -> Result<Price, PriceRepositoryErr> {
        let conn = get_database_connection().await;

        let data = PriceDb::find()
            .order_by(price_db::Column::Date, Order::Desc)
            .one(conn)
            .await
            .map_err(|_| PriceRepositoryErr::FailToGetLatest)?
            .ok_or(PriceRepositoryErr::FailToGetLatest)?;

        Ok(data.into())
    }
}

impl BaseRepository<Price> for PriceRepository {
    async fn create(&mut self, model: Price) -> Result<i64, BaseRepositoryErr> {
        let active_model = model.to_active_without_id();
        let result = PriceDb.create(active_model).await;

        match result {
            Ok(created_model) => Ok(created_model.price_id),
            Err(_) => Err(BaseRepositoryErr::FailToCreate),
        }
    }

    async fn read(&mut self, id: i64) -> Result<Option<Price>, BaseRepositoryErr> {
        match PriceDb.get_by_id(id).await {
            Ok(model) => {
                let model = model.ok_or(BaseRepositoryErr::FailToRead)?;
                Ok(Some(model.into()))
            }
            Err(_) => Err(BaseRepositoryErr::FailToCreate),
        }
    }

    async fn update(&mut self, model: Price) -> Result<Price, BaseRepositoryErr> {
        let active_model = model.to_active_with_id();
        let update_result = PriceDb.update_by_model(active_model).await;

        match update_result {
            Ok(model) => Ok(model.into()),
            Err(_) => Err(BaseRepositoryErr::FailToUpdate),
        }
    }

    async fn delete(&mut self, id: i64) -> Result<u64, BaseRepositoryErr> {
        match PriceDb.delete_by_model_id(id).await {
            Ok(deleted_count) => {
                if deleted_count > 0 {
                    return Ok(deleted_count);
                }

                Err(BaseRepositoryErr::FailToDelete)
            }
            Err(_) => Err(BaseRepositoryErr::FailToDelete),
        }
    }
}
