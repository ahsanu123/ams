use crate::migration::migration_trait::Migrationable;
use ams_macro::{GenerateDieselTable, GenerateTableEnum};
use diesel::prelude::*;
use sea_query::ForeignKeyAction;
use sea_query::{ColumnDef, ForeignKey, Iden, SchemaBuilder, Table};

#[derive(
    GenerateTableEnum,
    GenerateDieselTable,
    Queryable,
    Identifiable,
    Selectable,
    Debug,
    PartialEq,
    Insertable,
)]
#[diesel(table_name = book_table)]
pub struct Book {
    pub id: i32,
    pub title: String,
}

impl Migrationable for Book {
    fn get_up_migration(builder: impl SchemaBuilder) -> String {
        Table::create()
            .table(BookTable::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(BookTable::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new(BookTable::Title).text())
            .build(builder)
    }
    fn get_down_migration(builder: impl SchemaBuilder) -> String {
        Table::drop()
            .table(BookTable::Table)
            .if_exists()
            .build(builder)
    }
}

#[derive(
    GenerateTableEnum,
    GenerateDieselTable,
    Queryable,
    Identifiable,
    Associations,
    Selectable,
    Debug,
    PartialEq,
    Insertable,
)]
#[diesel(table_name = page_table)]
#[diesel(belongs_to(Book))]
pub struct Page {
    pub id: i32,
    pub book_id: i32,
    pub page_number: i32,
    pub content: String,
}

diesel::joinable!(page_table -> book_table(book_id));
diesel::allow_tables_to_appear_in_same_query!(book_table, page_table);

impl Migrationable for Page {
    fn get_up_migration(builder: impl SchemaBuilder) -> String {
        Table::create()
            .table(PageTable::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(PageTable::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new(PageTable::PageNumber).integer())
            .col(ColumnDef::new(PageTable::Content).text())
            .col(ColumnDef::new(PageTable::BookId).integer())
            .foreign_key(
                ForeignKey::create()
                    .name("FK_PageToBook")
                    .from(PageTable::Table, PageTable::BookId)
                    .to(BookTable::Table, BookTable::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .build(builder)
    }

    fn get_down_migration(builder: impl SchemaBuilder) -> String {
        Table::drop()
            .table(PageTable::Table)
            .if_exists()
            .build(builder)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::helper::sql_connection_helper::create_connection;
    use sea_query::SqliteQueryBuilder;

    #[test]
    fn diesel_relation_test() {
        let conn = &mut create_connection();
        let momo = book_table::table
            .filter(book_table::title.eq("Momo"))
            .select(Book::as_select())
            .get_result(conn)
            .unwrap();

        let pages = Page::belonging_to(&momo)
            .select(Page::as_select())
            .load(conn);

        println!("=====================================");
        println!("Found Book with title {}", momo.title);
        println!("Pages for Momo: {pages:#?}");
    }

    #[test]
    fn check_insert_book() {
        let conn = &mut create_connection();
        let book = Book {
            id: 0,
            title: String::from("Momo"),
        };

        let _result = diesel::insert_into(book_table::table)
            .values(&book)
            .execute(conn)
            .expect("error when inserting new book");
    }

    #[test]
    fn check_insert_pages() {
        let conn = &mut create_connection();

        let pages = vec![
            Page {
                id: 1,
                page_number: 1,
                content: String::from("Content of Page 1"),
                book_id: 0,
            },
            Page {
                id: 2,
                page_number: 2,
                content: String::from("Content of Page 2"),
                book_id: 0,
            },
            Page {
                id: 3,
                page_number: 3,
                content: String::from("Content of Page 3"),
                book_id: 0,
            },
        ];

        let _result = diesel::insert_into(page_table::table)
            .values(&pages)
            .execute(conn)
            .expect("error when inserting new pages!!!");
    }

    #[test]
    fn check_book_up_migration() {
        let conn = &mut create_connection();

        let book_up = Book::get_up_migration(SqliteQueryBuilder);
        let book_down = Book::get_down_migration(SqliteQueryBuilder);

        let page_up = Page::get_up_migration(SqliteQueryBuilder);
        let page_down = Page::get_down_migration(SqliteQueryBuilder);

        println!("book up \n {book_up}");
        println!("book down \n {book_down}");
        println!("page up \n {page_up}");
        println!("page down \n {page_down}");

        let _ = diesel::sql_query(book_up)
            .execute(conn)
            .expect("error when migrate up book!!!");

        let _ = diesel::sql_query(page_up)
            .execute(conn)
            .expect("error when migrate up page!!!");
    }
}
