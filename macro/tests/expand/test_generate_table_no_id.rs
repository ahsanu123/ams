#[macro_use]
extern crate ams_macro;

#[derive(GenerateNoIdStruct)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub description: String,
}
