use crate::models::Post;
use diesel::{backend::Backend, expression::Selectable, sqlite::Sqlite};

pub trait AbstractRepository<T, TBackend = Sqlite>
where
    TBackend: Backend,
    T: Selectable<TBackend>,
{
    fn insert(id: i64) -> T;
    fn get(id: i64) -> T;
    fn get_all() -> Vec<T>;
    fn update() -> T;
    fn delete(id: i64) -> u32;
}

struct PostRepository;

impl AbstractRepository<Post> for PostRepository {
    fn insert(id: i64) -> Post {
        todo!()
    }

    fn get(id: i64) -> Post {
        todo!()
    }

    fn get_all() -> Vec<Post> {
        todo!()
    }

    fn update() -> Post {
        todo!()
    }

    fn delete(id: i64) -> u32 {
        todo!()
    }
}
