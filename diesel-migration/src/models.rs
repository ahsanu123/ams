use crate::schema::posts;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = posts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[diesel(table_name=posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

mod generated {
    use super::*;
    use diesel::result::Error;

    impl posts::dsl::posts {
        fn get_by_id(self, id: i32, conn: &mut SqliteConnection) -> Result<Option<Post>, Error> {
            self.find(id)
                .select(Post::as_select())
                .first(conn)
                .optional()
        }
    }
}
