use crate::model::{user::User, *};

use super::crud_repository_trait::CrudRepositoryTrait;

pub struct UserRepository {}

impl CrudRepositoryTrait<User> for UserRepository {
    fn getAll(&self) -> Result<Vec<User>, String> {
        todo!()
    }

    fn create(&self, data: &User) -> Result<usize, String> {
        todo!()
    }

    fn read(&self, id: u32) -> Result<User, String> {
        todo!()
    }

    fn update(&self, data: &User) -> Result<usize, String> {
        todo!()
    }

    fn delete(&self, id: u32) -> Result<usize, String> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn user_repository_get_all() {
        todo!();
    }

    #[test]
    fn user_repository_create() {
        todo!();
    }

    #[test]
    fn user_repository_read() {
        todo!();
    }

    #[test]
    fn user_repository_update() {
        todo!();
    }

    #[test]
    fn user_repository_delete() {
        todo!();
    }
}
