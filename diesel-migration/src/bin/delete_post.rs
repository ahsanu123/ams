use diesel::prelude::*;
use diesel_migration::*;
use std::env::args;

fn main() {
    use self::schema::posts::dsl::*;

    let target_id = args().nth(1).expect("please insert correct post id");
    let pattern = format!("%{}%", target_id);

    let conn = &mut establish_connection();

    let deleted_count = diesel::delete(posts.filter(title.like(pattern)))
        .execute(conn)
        .expect("error when deleting post");

    println!("deleted {} posts", deleted_count);
}
