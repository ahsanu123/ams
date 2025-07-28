use diesel_migration::*;
use std::io::{Read, stdin};

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";

fn main() {
    let connection = &mut establish_connection();

    let mut title = String::new();
    let mut body = String::new();

    println!("insert title!");
    stdin().read_line(&mut title).unwrap();
    let title = title.trim_end();

    println!("insert body for {title}!! (press {EOF} to finish)");
    stdin().read_to_string(&mut body).unwrap();

    let post = create_post(connection, title, &body);
    println!("saved new post {title}, with id {}", post.id)
}
