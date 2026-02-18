pub enum BaseRepositoryErr {
    FailToCreate,
    FailToRead,
    FailToUpdate,
    FailToDelete,
    // ...
}
pub trait BaseRepository<T> {
    async fn create(&mut self, model: T) -> Result<i64, BaseRepositoryErr>;
    async fn read(&mut self, id: i64) -> Result<T, BaseRepositoryErr>;
    async fn update(&mut self, model: T) -> Result<T, BaseRepositoryErr>;
    async fn delete(&mut self, id: i64) -> Result<i64, BaseRepositoryErr>;
}
