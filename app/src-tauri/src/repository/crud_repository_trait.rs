pub trait CrudRepositoryTrait<T> {
    async fn create(&self, data: &T);
    async fn read(&self, id: u32);
    async fn update(&self, data: &T);
    async fn delete(&self, id: u32);
}

pub trait IntoValueAndColumnTrait<T, V> {
    fn columns() -> Vec<T>;
}
