use diesel::prelude::*;
use diesel_migration::{establish_connection, models::Post};
use std::env::args;

fn main() {
    use diesel_migration::schema::posts::dsl::{posts, published};
    let id = args()
        .nth(1)
        .expect("please give post id to be publish")
        .parse::<i32>()
        .expect("cant parse input, please provide valid id");

    let connection = &mut establish_connection();
    let post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .returning(Post::as_returning())
        .get_result(connection)
        .unwrap();

    println!("published post {}", post.title);
}
