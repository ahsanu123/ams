// use diesel::query_dsl::methods::{FindDsl, SelectDsl};
// use diesel::{OptionalExtension, RunQueryDsl, SelectableHelper};
use diesel::prelude::*;
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

    let post = posts
        .find(post_id)
        .select(Post::as_select())
        .first(connection)
        .optional();
    // This allows for returning an Option<Post>, otherwise it will throw an error

    match post {
        Ok(Some(post)) => println!("Post with id: {} has a title: {}", post.id, post.title),
        Ok(None) => println!("Unable to find post {}", post_id),
        Err(_) => println!("An error occured while fetching post {}", post_id),
    }
}

mod proto {
    use diesel::prelude::*;
    use diesel::query_dsl::methods::FindDsl;
    use diesel_migration::models::Post;
    use diesel_migration::*;

    pub trait IsQueryDsl<PK>
    where
        Self: FindDsl<PK>,
    {
        fn check_find(self, id: PK);
    }

    impl<T, PK> IsQueryDsl<PK> for T
    where
        Self: FindDsl<PK>,
    {
        fn check_find(self, id: PK) {
            // self.find(1).
            todo!()
        }
    }

    pub trait AbstractRepository<T>
    where
        T: diesel::Table,
        Self: diesel::Table,
    {
        type Output;

        fn get_byid<PK>(self, id: PK)
        where
            Self: Table + FindDsl<PK>;
    }

    impl<T> AbstractRepository<T> for T
    where
        T: Table,
    {
        type Output = i32;

        fn get_byid<PK>(self, id: PK)
        where
            Self: Table + QueryDsl,
        {
            todo!()
            // let selection = self.
            // let data = self.find(id).select();
        }
    }

    pub fn get() {
        use diesel_migration::schema::posts::dsl::posts;
        let connection = &mut establish_connection();

        // let p = posts.check_find();
        // let post = posts.get_byid::<Post>(1);
    }
}
