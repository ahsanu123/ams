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
        use diesel::row::{Field as _, Row as _};
        impl<__DB: diesel::backend::Backend, __ST0, __ST1, __ST2, __ST3>
            diesel::deserialize::Queryable<(__ST0, __ST1, __ST2, __ST3), __DB> for Post
        where
            (i32, String, String, bool):
                diesel::deserialize::FromStaticSqlRow<(__ST0, __ST1, __ST2, __ST3), __DB>,
        {
            type Row = (i32, String, String, bool);
            fn build(row: (i32, String, String, bool)) -> diesel::deserialize::Result<Self> {
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
        {
        }
    };
}
pub mod models {
    use crate::schema::posts;
    use diesel::prelude::*;
    #[diesel(table_name = posts)]
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
        use diesel::row::{Field as _, Row as _};
        impl<__DB: diesel::backend::Backend, __ST0, __ST1, __ST2, __ST3>
            diesel::deserialize::Queryable<(__ST0, __ST1, __ST2, __ST3), __DB> for Post
        where
            (i32, String, String, bool):
                diesel::deserialize::FromStaticSqlRow<(__ST0, __ST1, __ST2, __ST3), __DB>,
        {
            type Row = (i32, String, String, bool);
            fn build(row: (i32, String, String, bool)) -> diesel::deserialize::Result<Self> {
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
            type SelectExpression = (posts::id, posts::title, posts::body, posts::published);
            fn construct_selection() -> Self::SelectExpression {
                (posts::id, posts::title, posts::body, posts::published)
            }
        }
        fn _check_field_compatibility()
        where
            i32: diesel::deserialize::FromSqlRow<
                diesel::dsl::SqlTypeOf<posts::id>,
                diesel::sqlite::Sqlite,
            >,
            String: diesel::deserialize::FromSqlRow<
                diesel::dsl::SqlTypeOf<posts::title>,
                diesel::sqlite::Sqlite,
            >,
            String: diesel::deserialize::FromSqlRow<
                diesel::dsl::SqlTypeOf<posts::body>,
                diesel::sqlite::Sqlite,
            >,
            bool: diesel::deserialize::FromSqlRow<
                diesel::dsl::SqlTypeOf<posts::published>,
                diesel::sqlite::Sqlite,
            >,
        {
        }
    };
    #[diesel(table_name = posts)]
    pub struct NewPost<'a> {
        pub title: &'a str,
        pub body: &'a str,
    }
    #[allow(unused_imports)]
    const _: () = {
        use diesel;
        #[allow(unused_qualifications)]
        impl<'a> diesel::insertable::Insertable<posts::table> for NewPost<'a> {
            type Values = <(
                std::option::Option<diesel::dsl::Eq<posts::title, &'a str>>,
                std::option::Option<diesel::dsl::Eq<posts::body, &'a str>>,
            ) as diesel::insertable::Insertable<posts::table>>::Values;
            fn values(
                self,
            ) -> <(
                std::option::Option<diesel::dsl::Eq<posts::title, &'a str>>,
                std::option::Option<diesel::dsl::Eq<posts::body, &'a str>>,
            ) as diesel::insertable::Insertable<posts::table>>::Values {
                diesel::insertable::Insertable::<posts::table>::values((
                    std::option::Option::Some(diesel::ExpressionMethods::eq(
                        posts::title,
                        self.title,
                    )),
                    std::option::Option::Some(diesel::ExpressionMethods::eq(
                        posts::body,
                        self.body,
                    )),
                ))
            }
        }
        #[allow(unused_qualifications)]
        impl<'a, 'insert> diesel::insertable::Insertable<posts::table> for &'insert NewPost<'a> {
            type Values = <(
                std::option::Option<diesel::dsl::Eq<posts::title, &'insert &'a str>>,
                std::option::Option<diesel::dsl::Eq<posts::body, &'insert &'a str>>,
            ) as diesel::insertable::Insertable<posts::table>>::Values;
            fn values(
                self,
            ) -> <(
                std::option::Option<diesel::dsl::Eq<posts::title, &'insert &'a str>>,
                std::option::Option<diesel::dsl::Eq<posts::body, &'insert &'a str>>,
            ) as diesel::insertable::Insertable<posts::table>>::Values {
                diesel::insertable::Insertable::<posts::table>::values((
                    std::option::Option::Some(diesel::ExpressionMethods::eq(
                        posts::title,
                        &self.title,
                    )),
                    std::option::Option::Some(diesel::ExpressionMethods::eq(
                        posts::body,
                        &self.body,
                    )),
                ))
            }
        }
        impl<'a> diesel::internal::derives::insertable::UndecoratedInsertRecord<posts::table>
            for NewPost<'a>
        {
        }
    };
}
pub mod models {
    use crate::schema::posts;
    use diesel::prelude::*;
    #[diesel(table_name = posts)]
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
        use diesel::row::{Field as _, Row as _};
        impl<__DB: diesel::backend::Backend, __ST0, __ST1, __ST2, __ST3>
            diesel::deserialize::Queryable<(__ST0, __ST1, __ST2, __ST3), __DB> for Post
        where
            (i32, String, String, bool):
                diesel::deserialize::FromStaticSqlRow<(__ST0, __ST1, __ST2, __ST3), __DB>,
        {
            type Row = (i32, String, String, bool);
            fn build(row: (i32, String, String, bool)) -> diesel::deserialize::Result<Self> {
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
            type SelectExpression = (posts::id, posts::title, posts::body, posts::published);
            fn construct_selection() -> Self::SelectExpression {
                (posts::id, posts::title, posts::body, posts::published)
            }
        }
        fn _check_field_compatibility()
        where
            i32: diesel::deserialize::FromSqlRow<
                diesel::dsl::SqlTypeOf<posts::id>,
                diesel::sqlite::Sqlite,
            >,
            String: diesel::deserialize::FromSqlRow<
                diesel::dsl::SqlTypeOf<posts::title>,
                diesel::sqlite::Sqlite,
            >,
            String: diesel::deserialize::FromSqlRow<
                diesel::dsl::SqlTypeOf<posts::body>,
                diesel::sqlite::Sqlite,
            >,
            bool: diesel::deserialize::FromSqlRow<
                diesel::dsl::SqlTypeOf<posts::published>,
                diesel::sqlite::Sqlite,
            >,
        {
        }
    };
    #[diesel(table_name = posts)]
    pub struct NewPost<'a> {
        pub title: &'a str,
        pub body: &'a str,
    }
    #[allow(unused_imports)]
    const _: () = {
        use diesel;
        #[allow(unused_qualifications)]
        impl<'a> diesel::insertable::Insertable<posts::table> for NewPost<'a> {
            type Values = <(
                std::option::Option<diesel::dsl::Eq<posts::title, &'a str>>,
                std::option::Option<diesel::dsl::Eq<posts::body, &'a str>>,
            ) as diesel::insertable::Insertable<posts::table>>::Values;
            fn values(
                self,
            ) -> <(
                std::option::Option<diesel::dsl::Eq<posts::title, &'a str>>,
                std::option::Option<diesel::dsl::Eq<posts::body, &'a str>>,
            ) as diesel::insertable::Insertable<posts::table>>::Values {
                diesel::insertable::Insertable::<posts::table>::values((
                    std::option::Option::Some(diesel::ExpressionMethods::eq(
                        posts::title,
                        self.title,
                    )),
                    std::option::Option::Some(diesel::ExpressionMethods::eq(
                        posts::body,
                        self.body,
                    )),
                ))
            }
        }
        #[allow(unused_qualifications)]
        impl<'a, 'insert> diesel::insertable::Insertable<posts::table> for &'insert NewPost<'a> {
            type Values = <(
                std::option::Option<diesel::dsl::Eq<posts::title, &'insert &'a str>>,
                std::option::Option<diesel::dsl::Eq<posts::body, &'insert &'a str>>,
            ) as diesel::insertable::Insertable<posts::table>>::Values;
            fn values(
                self,
            ) -> <(
                std::option::Option<diesel::dsl::Eq<posts::title, &'insert &'a str>>,
                std::option::Option<diesel::dsl::Eq<posts::body, &'insert &'a str>>,
            ) as diesel::insertable::Insertable<posts::table>>::Values {
                diesel::insertable::Insertable::<posts::table>::values((
                    std::option::Option::Some(diesel::ExpressionMethods::eq(
                        posts::title,
                        &self.title,
                    )),
                    std::option::Option::Some(diesel::ExpressionMethods::eq(
                        posts::body,
                        &self.body,
                    )),
                ))
            }
        }
        impl<'a> diesel::internal::derives::insertable::UndecoratedInsertRecord<posts::table>
            for NewPost<'a>
        {
        }
    };
}
