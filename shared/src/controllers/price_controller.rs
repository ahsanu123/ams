use crate::{
    models::price::Price,
    repositories::{PRICE_REPO, base_repository_trait::BaseRepository},
};

pub trait PriceControllerTrait {
    fn get_latest_dreg_price(&mut self) -> impl Future<Output = Option<Price>>;
    fn update_dreg_price(&mut self, new_price: f32) -> impl Future<Output = i64>;
    fn get_all_dreg_price(&mut self) -> impl Future<Output = Vec<Price>>;
}

pub struct PriceController;

impl PriceControllerTrait for PriceController {
    async fn get_latest_dreg_price(&mut self) -> Option<Price> {
        PRICE_REPO.lock().await.get_latest().await.ok()
    }

    async fn update_dreg_price(&mut self, new_price: f32) -> i64 {
        PRICE_REPO
            .lock()
            .await
            .create(Price::new(new_price))
            .await
            .unwrap_or_default()
    }

    async fn get_all_dreg_price(&mut self) -> Vec<Price> {
        PRICE_REPO.lock().await.get_all().await.unwrap_or_default()
    }
}
