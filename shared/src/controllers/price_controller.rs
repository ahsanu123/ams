use serde::Serialize;

use crate::{
    models::price::Price,
    repositories::{PRICE_REPO, base_repository_trait::BaseRepository},
};

pub trait PriceControllerTrait {
    fn get_latest_dreg_price(&mut self) -> impl Future<Output = Result<Price, PriceControllerErr>>;

    fn update_dreg_price(
        &mut self,
        new_price: f32,
    ) -> impl Future<Output = Result<i64, PriceControllerErr>>;

    fn get_all_dreg_price(
        &mut self,
    ) -> impl Future<Output = Result<Vec<Price>, PriceControllerErr>>;

    fn delete_by_id(
        &mut self,
        price_id: i64,
    ) -> impl Future<Output = Result<u64, PriceControllerErr>>;
}

#[derive(Serialize)]
pub enum PriceControllerErr {
    FailGetLatest,
    FailUpdate,
    FailGetAll,
    FailDelete,
}
pub struct PriceController;

impl PriceControllerTrait for PriceController {
    async fn get_latest_dreg_price(&mut self) -> Result<Price, PriceControllerErr> {
        PRICE_REPO
            .lock()
            .await
            .get_latest()
            .await
            .map_err(|_| PriceControllerErr::FailGetLatest)
    }

    async fn update_dreg_price(&mut self, new_price: f32) -> Result<i64, PriceControllerErr> {
        PRICE_REPO
            .lock()
            .await
            .create(Price::new(new_price))
            .await
            .map_err(|_| PriceControllerErr::FailUpdate)
    }

    async fn get_all_dreg_price(&mut self) -> Result<Vec<Price>, PriceControllerErr> {
        PRICE_REPO
            .lock()
            .await
            .get_all()
            .await
            .map_err(|_| PriceControllerErr::FailGetAll)
    }

    async fn delete_by_id(&mut self, price_id: i64) -> Result<u64, PriceControllerErr> {
        PRICE_REPO
            .lock()
            .await
            .delete(price_id)
            .await
            .map_err(|_| PriceControllerErr::FailDelete)
    }
}
