use diesel::prelude::*;
use diesel_migration::{models::*, *};

fn main() {
    use self::schema::posts::dsl::*;

    let connection = &mut establish_connection();
    let result = posts
        .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(connection)
        .expect("error when loading posts.");

    println!("displaying {} posts", result.len());

    for post in result {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}
