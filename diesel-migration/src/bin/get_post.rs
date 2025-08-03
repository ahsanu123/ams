use diesel::{OptionalExtension as _, QueryDsl as _, RunQueryDsl as _, SelectableHelper as _};
use diesel_migration::models::Post;
use diesel_migration::*;
use std::env::args;

fn main() {
    use self::schema::posts::dsl::posts;

    let post_id = args()
        .nth(1)
        .expect("get_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");

    let connection = &mut establish_connection();

    let post = posts
        .find(post_id)
        .select(Post::as_select())
        .first(connection)
        .optional();
    // This allows for returning an Option<Post>, otherwise it will throw an error

    match post {
        Ok(Some(post)) => println!("Post with id: {} has a title: {}", post.id, post.title),
        Ok(None) => println!("Unable to find post {}", post_id),
        Err(_) => println!("An error occured while fetching post {}", post_id),
    }
}

mod proto {
    use diesel::{
        EqAll, IntoSql, OptionalExtension, QueryDsl, QueryResult, RunQueryDsl, Selectable,
        SelectableHelper, SqliteConnection, Table,
        backend::Backend,
        expression::AsExpression,
        query_builder::AsQuery,
        query_dsl::methods::{FindDsl, LimitDsl, LoadQuery, SelectDsl},
        sql_types::Integer,
        sqlite::Sqlite,
    };
    use diesel_migration::{establish_connection, models::Post, schema::posts::dsl::posts};

    pub trait AbstractRepository {
        fn get_byid(self, id: i64, conn: &mut SqliteConnection);
    }

    impl<'a, T> AbstractRepository for T
    where
        T: Table + FindDsl<i64>,
        <T as FindDsl<i64>>::Output: SelectDsl<<T as Table>::AllColumns>,
        <<T as FindDsl<i64>>::Output as SelectDsl<<T as Table>::AllColumns>>::Output: LimitDsl,
        <<<T as FindDsl<i64>>::Output as SelectDsl<<T as Table>::AllColumns>>::Output as LimitDsl>::Output: LoadQuery<'a, SqliteConnection,<T as Table>::AllColumns >
    {
        fn get_byid(self, id: i64, conn: &mut SqliteConnection) {
            let r = self
                .find(id)
                .select(T::all_columns())
                .limit(1)
                .get_result(conn);
        }
    }

    pub fn get_byid<'a, T, PK>(data: T, id: PK, conn: &mut SqliteConnection)
    where
        T: Table,
        T: FindDsl<PK> + AsQuery,
        T::PrimaryKey: EqAll<PK>,
    // Source as FindDsl<PK>>::Output;
    // <T as FindDsl<i64>>::Output:Find<Self, i64>,
        <T as FindDsl<PK>>::Output: SelectDsl<<T as Table>::AllColumns>,
        <<T as FindDsl<PK>>::Output as SelectDsl<<T as Table>::AllColumns>>::Output: LimitDsl,
        <<<T as FindDsl<PK>>::Output as SelectDsl<<T as Table>::AllColumns>>::Output as LimitDsl>::Output: LoadQuery<'a, SqliteConnection,<T as Table>::AllColumns >
    {
        let r = data
            .find(id)
            .select(T::all_columns())
            .limit(1)
            .get_result(conn);
    }

    pub trait EstablishConnectionTrait<Backend = SqliteConnection> {
        fn create_connection(self) -> Backend;
    }

    impl<T> EstablishConnectionTrait for T
    where
        T: Table,
    {
        fn create_connection(self) -> SqliteConnection {
            establish_connection()
        }
    }

    // pub fn find_by_id_optional<T, PK, S, M>(
    //     table: T,
    //     id: PK,
    //     conn: &mut SqliteConnection,
    // ) -> QueryResult<Option<M>>
    // where
    //     T: FindDsl<PK>,
    //     <T as FindDsl<PK>>::Output: SelectDsl<S>,
    //     <<T as FindDsl<PK>>::Output as SelectDsl<S>>::Output: LoadQuery<'_, SqliteConnection, M>,
    //     PK: diesel::expression::AsExpression<<T::Table as Table>::PrimaryKey::SqlType>,
    // {
    //     table
    //         .find(id)
    //         .select(S::default()) // OR pass `S` in as a parameter
    //         .first::<M>(conn)
    //         .optional()
    // }

    fn hoho() {
        let conn = &mut posts.create_connection();
        // let res = posts.get_byid(1);
        // let res = get_byid(
        //     1,
        //     conn,
        // );
    }
}
