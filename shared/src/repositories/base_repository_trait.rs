pub enum BaseRepositoryErr {
    FailToCreate,
    FailToRead,
    FailToUpdate,
    FailToDelete,
    FailToGetRelated,
    // ...
}
pub trait BaseRepository<T> {
    async fn create(&mut self, model: T) -> Result<i64, BaseRepositoryErr>;
    async fn read(&mut self, id: i64) -> Result<Option<T>, BaseRepositoryErr>;
    async fn update(&mut self, model: T) -> Result<T, BaseRepositoryErr>;
    async fn delete(&mut self, id: i64) -> Result<u64, BaseRepositoryErr>;
}
