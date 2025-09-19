use diesel::associations::HasTable;
use diesel::{OptionalExtension as _, QueryDsl as _, RunQueryDsl as _, SelectableHelper as _};
use diesel_migration::models::Post;
use diesel_migration::*;
use std::env::args;

fn main() {
    use self::schema::posts::dsl::posts;

    let post_id = args()
        .nth(1)
        .expect("get_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");

    let connection = &mut establish_connection();

    let po = posts::table();

    let post = posts
        .find(post_id)
        .select(Post::as_select())
        .first(connection)
        .optional();

    match post {
        Ok(Some(post)) => println!("Post with id: {} has a title: {}", post.id, post.title),
        Ok(None) => println!("Unable to find post {}", post_id),
        Err(_) => println!("An error occured while fetching post {}", post_id),
    }
}

mod proto {
    use diesel::{associations::HasTable, prelude::*, result::Error, sqlite::Sqlite};

    pub trait Repository<T>: HasTable
    where
        T: Queryable<<T as Selectable<Sqlite>>::SelectExpression, Sqlite>
            + Selectable<Sqlite>
            + Sized,
    {
        fn get_by_id(conn: &mut SqliteConnection, id: i32) -> Result<T, Error> {
            todo!()
        }
    }
}
