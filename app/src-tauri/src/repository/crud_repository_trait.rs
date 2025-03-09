pub trait CrudRepositoryTrait<T> {
    fn getAll(&self) -> Result<Vec<T>, String>;
    fn create(&self, data: &T) -> Result<usize, String>;
    fn read(&self, id: u32) -> Result<T, String>;
    fn update(&self, data: &T) -> Result<usize, String>;
    fn delete(&self, id: u32) -> Result<usize, String>;
}
