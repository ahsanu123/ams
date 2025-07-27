#[macro_use]
extern crate ams_macro;
pub struct Book {
    pub id: i32,
    pub title: String,
}
pub enum BookTable {
    Id,
    Title,
    Table,
}
#[automatically_derived]
impl ::core::fmt::Debug for BookTable {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                BookTable::Id => "Id",
                BookTable::Title => "Title",
                BookTable::Table => "Table",
            },
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for BookTable {
    #[inline]
    fn clone(&self) -> BookTable {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for BookTable {}
