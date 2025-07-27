#[macro_use]
extern crate ams_macro;

#[derive(GenerateTableEnum)]
pub struct Book {
    pub id: i32,
    pub title: String,
}
