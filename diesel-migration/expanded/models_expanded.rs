pub mod models {
    use diesel::prelude::*;
    #[diesel(table_name = crate::schema::posts)]
    #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    pub struct Post {
        pub id: i32,
        pub title: String,
        pub body: String,
        pub published: bool,
    }
    #[allow(unused_imports)]
    const _: () = {
        use diesel;
        use diesel::row::{Row as _, Field as _};
        impl<
            __DB: diesel::backend::Backend,
            __ST0,
            __ST1,
            __ST2,
            __ST3,
        > diesel::deserialize::Queryable<(__ST0, __ST1, __ST2, __ST3), __DB> for Post
        where
            (
                i32,
                String,
                String,
                bool,
            ): diesel::deserialize::FromStaticSqlRow<(__ST0, __ST1, __ST2, __ST3), __DB>,
        {
            type Row = (i32, String, String, bool);
            fn build(
                row: (i32, String, String, bool),
            ) -> diesel::deserialize::Result<Self> {
                Ok(Self {
                    id: std::convert::TryInto::try_into(row.0)?,
                    title: std::convert::TryInto::try_into(row.1)?,
                    body: std::convert::TryInto::try_into(row.2)?,
                    published: std::convert::TryInto::try_into(row.3)?,
                })
            }
        }
    };
    #[allow(unused_imports)]
    const _: () = {
        use diesel;
        use diesel::expression::Selectable;
        impl<__DB: diesel::backend::Backend> Selectable<__DB> for Post {
            type SelectExpression = (
                crate::schema::posts::id,
                crate::schema::posts::title,
                crate::schema::posts::body,
                crate::schema::posts::published,
            );
            fn construct_selection() -> Self::SelectExpression {
                (
                    crate::schema::posts::id,
                    crate::schema::posts::title,
                    crate::schema::posts::body,
                    crate::schema::posts::published,
                )
            }
        }
        fn _check_field_compatibility()
        where
            i32: diesel::deserialize::FromSqlRow<
                diesel::dsl::SqlTypeOf<crate::schema::posts::id>,
                diesel::sqlite::Sqlite,
            >,
            String: diesel::deserialize::FromSqlRow<
                diesel::dsl::SqlTypeOf<crate::schema::posts::title>,
                diesel::sqlite::Sqlite,
            >,
            String: diesel::deserialize::FromSqlRow<
                diesel::dsl::SqlTypeOf<crate::schema::posts::body>,
                diesel::sqlite::Sqlite,
            >,
            bool: diesel::deserialize::FromSqlRow<
                diesel::dsl::SqlTypeOf<crate::schema::posts::published>,
                diesel::sqlite::Sqlite,
            >,
        {}
    };
}
