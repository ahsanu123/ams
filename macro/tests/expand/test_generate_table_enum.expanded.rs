#[macro_use]
extern crate ams_macro;
pub struct Book {
    pub id: i32,
    pub title: String,
}
use sea_query::Iden;
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
impl sea_query::Iden for BookTable {
    fn prepare(&self, s: &mut dyn ::std::fmt::Write, q: sea_query::Quote) {
        s.write_fmt(format_args!("{0}", q.left())).unwrap();
        self.unquoted(s);
        s.write_fmt(format_args!("{0}", q.right())).unwrap();
    }
    fn unquoted(&self, s: &mut dyn ::std::fmt::Write) {
        match self {
            Self::Id => s.write_fmt(format_args!("{0}", "id")).unwrap(),
            Self::Title => s.write_fmt(format_args!("{0}", "title")).unwrap(),
            Self::Table => s.write_fmt(format_args!("{0}", "book_table")).unwrap(),
        };
    }
}
