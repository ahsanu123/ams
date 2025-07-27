#[macro_use]
extern crate ams_macro;

#[derive(GenerateDieselTable)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub description: String,
}
