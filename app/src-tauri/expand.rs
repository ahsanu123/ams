pub mod book {
    use crate::migration::migration_trait::Migrationable;
    use custom_macro::{GenerateDieselTable, GenerateTableEnum};
    use diesel::{dsl::network, prelude::*};
    use sea_orm::ForeignKeyAction;
    use sea_query::{ColumnDef, ForeignKey, Iden, SchemaBuilder, Table};
    #[diesel(table_name = book_table)]
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
    #[allow(unused_imports, dead_code, unreachable_pub, unused_qualifications)]
    pub mod book_table {
        pub use self::columns::*;
        use ::diesel;
        use diesel::sql_types::*;
        /// Re-exports all of the columns of this table, as well as the
        /// table struct renamed to the module name. This is meant to be
        /// glob imported for functions which only deal with one table.
        pub mod dsl {
            pub use super::columns::id;
            pub use super::columns::title;
            pub use super::table as book_table;
        }
        #[allow(non_upper_case_globals, dead_code)]
        /// A tuple of all of the columns on this table
        pub const all_columns: (id, title) = (id, title);
        #[allow(non_camel_case_types)]
        /// The actual table struct
        ///
        /// This is the type which provides the base methods of the query
        /// builder, such as `.select` and `.filter`.
        pub struct table;
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::fmt::Debug for table {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "table")
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::clone::Clone for table {
            #[inline]
            fn clone(&self) -> table {
                *self
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::marker::Copy for table {}
        #[allow(unused_imports)]
        const _: () = {
            use diesel;
            use diesel::query_builder::QueryId;
            #[allow(non_camel_case_types)]
            impl QueryId for table {
                type QueryId = table;
                const HAS_STATIC_QUERY_ID: bool = true;
            }
        };
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::default::Default for table {
            #[inline]
            fn default() -> table {
                table {}
            }
        }
        impl table {
            #[allow(dead_code)]
            /// Represents `table_name.*`, which is sometimes necessary
            /// for efficient count queries. It cannot be used in place of
            /// `all_columns`
            pub fn star(&self) -> star {
                star
            }
        }
        /// The SQL type of all of the columns on this table
        pub type SqlType = (Integer, Text);
        /// Helper type for representing a boxed query from this table
        pub type BoxedQuery<'a, DB, ST = SqlType> =
            diesel::internal::table_macro::BoxedSelectStatement<
                'a,
                ST,
                diesel::internal::table_macro::FromClause<table>,
                DB,
            >;
        impl diesel::QuerySource for table {
            type FromClause = diesel::internal::table_macro::StaticQueryFragmentInstance<table>;
            type DefaultSelection = <Self as diesel::Table>::AllColumns;
            fn from_clause(&self) -> Self::FromClause {
                diesel::internal::table_macro::StaticQueryFragmentInstance::new()
            }
            fn default_selection(&self) -> Self::DefaultSelection {
                use diesel::Table;
                Self::all_columns()
            }
        }
        impl<DB> diesel::query_builder::QueryFragment<DB> for table
        where
            DB: diesel::backend::Backend,
            <table as diesel::internal::table_macro::StaticQueryFragment>::Component:
                diesel::query_builder::QueryFragment<DB>,
        {
            fn walk_ast<'b>(
                &'b self,
                __diesel_internal_pass: diesel::query_builder::AstPass<'_, 'b, DB>,
            ) -> diesel::result::QueryResult<()> {
                <table as diesel::internal::table_macro::StaticQueryFragment>::STATIC_COMPONENT
                    .walk_ast(__diesel_internal_pass)
            }
        }
        impl diesel::internal::table_macro::StaticQueryFragment for table {
            type Component = diesel::internal::table_macro::Identifier<'static>;
            const STATIC_COMPONENT: &'static Self::Component =
                &diesel::internal::table_macro::Identifier("book_table");
        }
        impl diesel::query_builder::AsQuery for table {
            type SqlType = SqlType;
            type Query = diesel::internal::table_macro::SelectStatement<
                diesel::internal::table_macro::FromClause<Self>,
            >;
            fn as_query(self) -> Self::Query {
                diesel::internal::table_macro::SelectStatement::simple(self)
            }
        }
        impl diesel::Table for table {
            type PrimaryKey = id;
            type AllColumns = (id, title);
            fn primary_key(&self) -> Self::PrimaryKey {
                id
            }
            fn all_columns() -> Self::AllColumns {
                (id, title)
            }
        }
        impl diesel::associations::HasTable for table {
            type Table = Self;
            fn table() -> Self::Table {
                table
            }
        }
        impl diesel::query_builder::IntoUpdateTarget for table {
            type WhereClause = <<Self as diesel::query_builder::AsQuery>::Query as diesel::query_builder::IntoUpdateTarget>::WhereClause;
            fn into_update_target(
                self,
            ) -> diesel::query_builder::UpdateTarget<Self::Table, Self::WhereClause> {
                use diesel::query_builder::AsQuery;
                let q: diesel::internal::table_macro::SelectStatement<
                    diesel::internal::table_macro::FromClause<table>,
                > = self.as_query();
                q.into_update_target()
            }
        }
        impl diesel::query_source::AppearsInFromClause<table> for table {
            type Count = diesel::query_source::Once;
        }
        impl<S> diesel::internal::table_macro::AliasAppearsInFromClause<S, table> for table
        where
            S: diesel::query_source::AliasSource<Target = table>,
        {
            type Count = diesel::query_source::Never;
        }
        impl<S1, S2> diesel::internal::table_macro::AliasAliasAppearsInFromClause<table, S2, S1> for table
        where
            S1: diesel::query_source::AliasSource<Target = table>,
            S2: diesel::query_source::AliasSource<Target = table>,
            S1: diesel::internal::table_macro::AliasAliasAppearsInFromClauseSameTable<S2, table>,
        {
            type Count =
                <S1 as diesel::internal::table_macro::AliasAliasAppearsInFromClauseSameTable<
                    S2,
                    table,
                >>::Count;
        }
        impl<S> diesel::query_source::AppearsInFromClause<diesel::query_source::Alias<S>> for table
        where
            S: diesel::query_source::AliasSource,
        {
            type Count = diesel::query_source::Never;
        }
        impl<S, C>
            diesel::internal::table_macro::FieldAliasMapperAssociatedTypesDisjointnessTrick<
                table,
                S,
                C,
            > for table
        where
            S: diesel::query_source::AliasSource<Target = table> + ::std::clone::Clone,
            C: diesel::query_source::Column<Table = table>,
        {
            type Out = diesel::query_source::AliasedField<S, C>;
            fn map(
                __diesel_internal_column: C,
                __diesel_internal_alias: &diesel::query_source::Alias<S>,
            ) -> Self::Out {
                __diesel_internal_alias.field(__diesel_internal_column)
            }
        }
        impl diesel::query_source::AppearsInFromClause<table>
            for diesel::internal::table_macro::NoFromClause
        {
            type Count = diesel::query_source::Never;
        }
        impl<Left, Right, Kind>
            diesel::JoinTo<diesel::internal::table_macro::Join<Left, Right, Kind>> for table
        where
            diesel::internal::table_macro::Join<Left, Right, Kind>: diesel::JoinTo<table>,
            Left: diesel::query_source::QuerySource,
            Right: diesel::query_source::QuerySource,
        {
            type FromClause = diesel::internal::table_macro::Join<Left, Right, Kind>;
            type OnClause = <diesel::internal::table_macro::Join<
                Left,
                Right,
                Kind,
            > as diesel::JoinTo<table>>::OnClause;
            fn join_target(
                __diesel_internal_rhs: diesel::internal::table_macro::Join<Left, Right, Kind>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, __diesel_internal_on_clause) =
                    diesel::internal::table_macro::Join::join_target(table);
                (__diesel_internal_rhs, __diesel_internal_on_clause)
            }
        }
        impl<Join, On> diesel::JoinTo<diesel::internal::table_macro::JoinOn<Join, On>> for table
        where
            diesel::internal::table_macro::JoinOn<Join, On>: diesel::JoinTo<table>,
        {
            type FromClause = diesel::internal::table_macro::JoinOn<Join, On>;
            type OnClause = <diesel::internal::table_macro::JoinOn<Join, On> as diesel::JoinTo<
                table,
            >>::OnClause;
            fn join_target(
                __diesel_internal_rhs: diesel::internal::table_macro::JoinOn<Join, On>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, __diesel_internal_on_clause) =
                    diesel::internal::table_macro::JoinOn::join_target(table);
                (__diesel_internal_rhs, __diesel_internal_on_clause)
            }
        }
        impl<F, S, D, W, O, L, Of, G>
            diesel::JoinTo<
                diesel::internal::table_macro::SelectStatement<
                    diesel::internal::table_macro::FromClause<F>,
                    S,
                    D,
                    W,
                    O,
                    L,
                    Of,
                    G,
                >,
            > for table
        where
            diesel::internal::table_macro::SelectStatement<
                diesel::internal::table_macro::FromClause<F>,
                S,
                D,
                W,
                O,
                L,
                Of,
                G,
            >: diesel::JoinTo<table>,
            F: diesel::query_source::QuerySource,
        {
            type FromClause = diesel::internal::table_macro::SelectStatement<
                diesel::internal::table_macro::FromClause<F>,
                S,
                D,
                W,
                O,
                L,
                Of,
                G,
            >;
            type OnClause = <diesel::internal::table_macro::SelectStatement<
                diesel::internal::table_macro::FromClause<F>,
                S,
                D,
                W,
                O,
                L,
                Of,
                G,
            > as diesel::JoinTo<table>>::OnClause;
            fn join_target(
                __diesel_internal_rhs: diesel::internal::table_macro::SelectStatement<
                    diesel::internal::table_macro::FromClause<F>,
                    S,
                    D,
                    W,
                    O,
                    L,
                    Of,
                    G,
                >,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, __diesel_internal_on_clause) =
                    diesel::internal::table_macro::SelectStatement::join_target(table);
                (__diesel_internal_rhs, __diesel_internal_on_clause)
            }
        }
        impl<'a, QS, ST, DB>
            diesel::JoinTo<
                diesel::internal::table_macro::BoxedSelectStatement<
                    'a,
                    diesel::internal::table_macro::FromClause<QS>,
                    ST,
                    DB,
                >,
            > for table
        where
            diesel::internal::table_macro::BoxedSelectStatement<
                'a,
                diesel::internal::table_macro::FromClause<QS>,
                ST,
                DB,
            >: diesel::JoinTo<table>,
            QS: diesel::query_source::QuerySource,
        {
            type FromClause = diesel::internal::table_macro::BoxedSelectStatement<
                'a,
                diesel::internal::table_macro::FromClause<QS>,
                ST,
                DB,
            >;
            type OnClause = <diesel::internal::table_macro::BoxedSelectStatement<
                'a,
                diesel::internal::table_macro::FromClause<QS>,
                ST,
                DB,
            > as diesel::JoinTo<table>>::OnClause;
            fn join_target(
                __diesel_internal_rhs: diesel::internal::table_macro::BoxedSelectStatement<
                    'a,
                    diesel::internal::table_macro::FromClause<QS>,
                    ST,
                    DB,
                >,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, __diesel_internal_on_clause) =
                    diesel::internal::table_macro::BoxedSelectStatement::join_target(table);
                (__diesel_internal_rhs, __diesel_internal_on_clause)
            }
        }
        impl<S> diesel::JoinTo<diesel::query_source::Alias<S>> for table
        where
            diesel::query_source::Alias<S>: diesel::JoinTo<table>,
        {
            type FromClause = diesel::query_source::Alias<S>;
            type OnClause = <diesel::query_source::Alias<S> as diesel::JoinTo<table>>::OnClause;
            fn join_target(
                __diesel_internal_rhs: diesel::query_source::Alias<S>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, __diesel_internal_on_clause) =
                    diesel::query_source::Alias::<S>::join_target(table);
                (__diesel_internal_rhs, __diesel_internal_on_clause)
            }
        }
        impl<T> diesel::insertable::Insertable<T> for table
        where
            <table as diesel::query_builder::AsQuery>::Query: diesel::insertable::Insertable<T>,
        {
            type Values = <<table as diesel::query_builder::AsQuery>::Query as diesel::insertable::Insertable<
                T,
            >>::Values;
            fn values(self) -> Self::Values {
                use diesel::query_builder::AsQuery;
                self.as_query().values()
            }
        }
        impl<'a, T> diesel::insertable::Insertable<T> for &'a table
        where
            table: diesel::insertable::Insertable<T>,
        {
            type Values = <table as diesel::insertable::Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                (*self).values()
            }
        }
        impl<S> diesel::JoinTo<diesel::query_builder::Only<S>> for table
        where
            diesel::query_builder::Only<S>: diesel::JoinTo<table>,
        {
            type FromClause = diesel::query_builder::Only<S>;
            type OnClause = <diesel::query_builder::Only<S> as diesel::JoinTo<table>>::OnClause;
            fn join_target(
                __diesel_internal_rhs: diesel::query_builder::Only<S>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, __diesel_internal_on_clause) =
                    diesel::query_builder::Only::<S>::join_target(table);
                (__diesel_internal_rhs, __diesel_internal_on_clause)
            }
        }
        impl diesel::query_source::AppearsInFromClause<diesel::query_builder::Only<table>> for table {
            type Count = diesel::query_source::Once;
        }
        impl diesel::query_source::AppearsInFromClause<table> for diesel::query_builder::Only<table> {
            type Count = diesel::query_source::Once;
        }
        impl<S, TSM> diesel::JoinTo<diesel::query_builder::Tablesample<S, TSM>> for table
        where
            diesel::query_builder::Tablesample<S, TSM>: diesel::JoinTo<table>,
            TSM: diesel::internal::table_macro::TablesampleMethod,
        {
            type FromClause = diesel::query_builder::Tablesample<S, TSM>;
            type OnClause =
                <diesel::query_builder::Tablesample<S, TSM> as diesel::JoinTo<table>>::OnClause;
            fn join_target(
                __diesel_internal_rhs: diesel::query_builder::Tablesample<S, TSM>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, __diesel_internal_on_clause) =
                    diesel::query_builder::Tablesample::<S, TSM>::join_target(table);
                (__diesel_internal_rhs, __diesel_internal_on_clause)
            }
        }
        impl<TSM>
            diesel::query_source::AppearsInFromClause<
                diesel::query_builder::Tablesample<table, TSM>,
            > for table
        where
            TSM: diesel::internal::table_macro::TablesampleMethod,
        {
            type Count = diesel::query_source::Once;
        }
        impl<TSM> diesel::query_source::AppearsInFromClause<table>
            for diesel::query_builder::Tablesample<table, TSM>
        where
            TSM: diesel::internal::table_macro::TablesampleMethod,
        {
            type Count = diesel::query_source::Once;
        }
        /// Contains all of the columns of this table
        pub mod columns {
            use super::table;
            use ::diesel;
            use diesel::sql_types::*;
            #[allow(non_camel_case_types, dead_code)]
            /// Represents `table_name.*`, which is sometimes needed for
            /// efficient count queries. It cannot be used in place of
            /// `all_columns`, and has a `SqlType` of `()` to prevent it
            /// being used that way
            pub struct star;
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for star {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "star")
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for star {
                #[inline]
                fn clone(&self) -> star {
                    *self
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for star {}
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for star {
                    type QueryId = star;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            };
            impl<__GB> diesel::expression::ValidGrouping<__GB> for star
            where
                (id, title): diesel::expression::ValidGrouping<__GB>,
            {
                type IsAggregate =
                    <(id, title) as diesel::expression::ValidGrouping<__GB>>::IsAggregate;
            }
            impl diesel::Expression for star {
                type SqlType = diesel::expression::expression_types::NotSelectable;
            }
            impl<DB: diesel::backend::Backend> diesel::query_builder::QueryFragment<DB> for star
            where
                <table as diesel::QuerySource>::FromClause:
                    diesel::query_builder::QueryFragment<DB>,
            {
                #[allow(non_snake_case)]
                fn walk_ast<'b>(
                    &'b self,
                    mut __diesel_internal_out: diesel::query_builder::AstPass<'_, 'b, DB>,
                ) -> diesel::result::QueryResult<()> {
                    use diesel::QuerySource;
                    if !__diesel_internal_out.should_skip_from() {
                        const FROM_CLAUSE:
                            diesel::internal::table_macro::StaticQueryFragmentInstance<table> =
                            diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                        FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                        __diesel_internal_out.push_sql(".");
                    }
                    __diesel_internal_out.push_sql("*");
                    Ok(())
                }
            }
            impl diesel::SelectableExpression<table> for star {}
            impl diesel::AppearsOnTable<table> for star {}
            #[allow(non_camel_case_types, dead_code)]
            pub struct id;
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for id {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "id")
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for id {
                #[inline]
                fn clone(&self) -> id {
                    *self
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for id {}
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for id {
                    type QueryId = id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            };
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for id {
                #[inline]
                fn default() -> id {
                    id {}
                }
            }
            impl diesel::expression::Expression for id {
                type SqlType = Integer;
            }
            impl<DB> diesel::query_builder::QueryFragment<DB> for id
            where
                DB: diesel::backend::Backend,
                diesel::internal::table_macro::StaticQueryFragmentInstance<table>:
                    diesel::query_builder::QueryFragment<DB>,
            {
                #[allow(non_snake_case)]
                fn walk_ast<'b>(
                    &'b self,
                    mut __diesel_internal_out: diesel::query_builder::AstPass<'_, 'b, DB>,
                ) -> diesel::result::QueryResult<()> {
                    if !__diesel_internal_out.should_skip_from() {
                        const FROM_CLAUSE:
                            diesel::internal::table_macro::StaticQueryFragmentInstance<table> =
                            diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                        FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                        __diesel_internal_out.push_sql(".");
                    }
                    __diesel_internal_out.push_identifier("id")
                }
            }
            impl diesel::SelectableExpression<super::table> for id {}
            impl<QS> diesel::AppearsOnTable<QS> for id where
                QS: diesel::query_source::AppearsInFromClause<
                    super::table,
                    Count = diesel::query_source::Once,
                >
            {
            }
            impl<Left, Right>
                diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for id
            where
                id: diesel::AppearsOnTable<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                >,
                Self: diesel::SelectableExpression<Left>,
                Right: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Never,
                    > + diesel::query_source::QuerySource,
                Left: diesel::query_source::QuerySource,
            {
            }
            impl<Left, Right>
                diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for id
            where
                id: diesel::AppearsOnTable<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                >,
                Left: diesel::query_source::AppearsInFromClause<super::table>
                    + diesel::query_source::QuerySource,
                Right: diesel::query_source::AppearsInFromClause<super::table>
                    + diesel::query_source::QuerySource,
                (Left::Count, Right::Count): diesel::internal::table_macro::Pick<Left, Right>,
                Self: diesel::SelectableExpression<
                    <(Left::Count, Right::Count) as diesel::internal::table_macro::Pick<
                        Left,
                        Right,
                    >>::Selection,
                >,
            {
            }
            impl<Join, On>
                diesel::SelectableExpression<diesel::internal::table_macro::JoinOn<Join, On>> for id
            where
                id: diesel::SelectableExpression<Join>
                    + diesel::AppearsOnTable<diesel::internal::table_macro::JoinOn<Join, On>>,
            {
            }
            impl<From>
                diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for id
            where
                From: diesel::query_source::QuerySource,
                id: diesel::SelectableExpression<From>
                    + diesel::AppearsOnTable<
                        diesel::internal::table_macro::SelectStatement<
                            diesel::internal::table_macro::FromClause<From>,
                        >,
                    >,
            {
            }
            impl<__GB> diesel::expression::ValidGrouping<__GB> for id
            where
                __GB: diesel::expression::IsContainedInGroupBy<
                    id,
                    Output = diesel::expression::is_contained_in_group_by::Yes,
                >,
            {
                type IsAggregate = diesel::expression::is_aggregate::Yes;
            }
            impl diesel::expression::ValidGrouping<()> for id {
                type IsAggregate = diesel::expression::is_aggregate::No;
            }
            impl diesel::expression::IsContainedInGroupBy<id> for id {
                type Output = diesel::expression::is_contained_in_group_by::Yes;
            }
            impl diesel::query_source::Column for id {
                type Table = super::table;
                const NAME: &'static str = "id";
            }
            impl<T> diesel::EqAll<T> for id
            where
                T: diesel::expression::AsExpression<Integer>,
                diesel::dsl::Eq<id, T::Expression>:
                    diesel::Expression<SqlType = diesel::sql_types::Bool>,
            {
                type Output = diesel::dsl::Eq<Self, T::Expression>;
                fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                    use diesel::expression_methods::ExpressionMethods;
                    self.eq(__diesel_internal_rhs)
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for id
            where
                Rhs: diesel::expression::AsExpression<
                    <<id as diesel::Expression>::SqlType as diesel::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = diesel::internal::table_macro::ops::Add<Self, Rhs::Expression>;
                fn add(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                    diesel::internal::table_macro::ops::Add::new(
                        self,
                        __diesel_internal_rhs.as_expression(),
                    )
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for id
            where
                Rhs: diesel::expression::AsExpression<
                    <<id as diesel::Expression>::SqlType as diesel::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = diesel::internal::table_macro::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                    diesel::internal::table_macro::ops::Sub::new(
                        self,
                        __diesel_internal_rhs.as_expression(),
                    )
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for id
            where
                Rhs: diesel::expression::AsExpression<
                    <<id as diesel::Expression>::SqlType as diesel::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = diesel::internal::table_macro::ops::Div<Self, Rhs::Expression>;
                fn div(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                    diesel::internal::table_macro::ops::Div::new(
                        self,
                        __diesel_internal_rhs.as_expression(),
                    )
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for id
            where
                Rhs: diesel::expression::AsExpression<
                    <<id as diesel::Expression>::SqlType as diesel::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = diesel::internal::table_macro::ops::Mul<Self, Rhs::Expression>;
                fn mul(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                    diesel::internal::table_macro::ops::Mul::new(
                        self,
                        __diesel_internal_rhs.as_expression(),
                    )
                }
            }
            impl
                diesel::query_source::AppearsInFromClause<diesel::query_builder::Only<super::table>>
                for id
            {
                type Count = diesel::query_source::Once;
            }
            impl diesel::SelectableExpression<diesel::query_builder::Only<super::table>> for id {}
            impl<TSM>
                diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for id
            where
                TSM: diesel::internal::table_macro::TablesampleMethod,
            {
                type Count = diesel::query_source::Once;
            }
            impl<TSM>
                diesel::SelectableExpression<diesel::query_builder::Tablesample<super::table, TSM>>
                for id
            where
                TSM: diesel::internal::table_macro::TablesampleMethod,
            {
            }
            #[allow(non_camel_case_types, dead_code)]
            pub struct title;
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for title {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "title")
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for title {
                #[inline]
                fn clone(&self) -> title {
                    *self
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for title {}
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for title {
                    type QueryId = title;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            };
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for title {
                #[inline]
                fn default() -> title {
                    title {}
                }
            }
            impl diesel::expression::Expression for title {
                type SqlType = Text;
            }
            impl<DB> diesel::query_builder::QueryFragment<DB> for title
            where
                DB: diesel::backend::Backend,
                diesel::internal::table_macro::StaticQueryFragmentInstance<table>:
                    diesel::query_builder::QueryFragment<DB>,
            {
                #[allow(non_snake_case)]
                fn walk_ast<'b>(
                    &'b self,
                    mut __diesel_internal_out: diesel::query_builder::AstPass<'_, 'b, DB>,
                ) -> diesel::result::QueryResult<()> {
                    if !__diesel_internal_out.should_skip_from() {
                        const FROM_CLAUSE:
                            diesel::internal::table_macro::StaticQueryFragmentInstance<table> =
                            diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                        FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                        __diesel_internal_out.push_sql(".");
                    }
                    __diesel_internal_out.push_identifier("title")
                }
            }
            impl diesel::SelectableExpression<super::table> for title {}
            impl<QS> diesel::AppearsOnTable<QS> for title where
                QS: diesel::query_source::AppearsInFromClause<
                    super::table,
                    Count = diesel::query_source::Once,
                >
            {
            }
            impl<Left, Right>
                diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for title
            where
                title: diesel::AppearsOnTable<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                >,
                Self: diesel::SelectableExpression<Left>,
                Right: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Never,
                    > + diesel::query_source::QuerySource,
                Left: diesel::query_source::QuerySource,
            {
            }
            impl<Left, Right>
                diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for title
            where
                title: diesel::AppearsOnTable<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                >,
                Left: diesel::query_source::AppearsInFromClause<super::table>
                    + diesel::query_source::QuerySource,
                Right: diesel::query_source::AppearsInFromClause<super::table>
                    + diesel::query_source::QuerySource,
                (Left::Count, Right::Count): diesel::internal::table_macro::Pick<Left, Right>,
                Self: diesel::SelectableExpression<
                    <(Left::Count, Right::Count) as diesel::internal::table_macro::Pick<
                        Left,
                        Right,
                    >>::Selection,
                >,
            {
            }
            impl<Join, On>
                diesel::SelectableExpression<diesel::internal::table_macro::JoinOn<Join, On>>
                for title
            where
                title: diesel::SelectableExpression<Join>
                    + diesel::AppearsOnTable<diesel::internal::table_macro::JoinOn<Join, On>>,
            {
            }
            impl<From>
                diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for title
            where
                From: diesel::query_source::QuerySource,
                title: diesel::SelectableExpression<From>
                    + diesel::AppearsOnTable<
                        diesel::internal::table_macro::SelectStatement<
                            diesel::internal::table_macro::FromClause<From>,
                        >,
                    >,
            {
            }
            impl<__GB> diesel::expression::ValidGrouping<__GB> for title
            where
                __GB: diesel::expression::IsContainedInGroupBy<
                    title,
                    Output = diesel::expression::is_contained_in_group_by::Yes,
                >,
            {
                type IsAggregate = diesel::expression::is_aggregate::Yes;
            }
            impl diesel::expression::ValidGrouping<()> for title {
                type IsAggregate = diesel::expression::is_aggregate::No;
            }
            impl diesel::expression::IsContainedInGroupBy<title> for title {
                type Output = diesel::expression::is_contained_in_group_by::Yes;
            }
            impl diesel::query_source::Column for title {
                type Table = super::table;
                const NAME: &'static str = "title";
            }
            impl<T> diesel::EqAll<T> for title
            where
                T: diesel::expression::AsExpression<Text>,
                diesel::dsl::Eq<title, T::Expression>:
                    diesel::Expression<SqlType = diesel::sql_types::Bool>,
            {
                type Output = diesel::dsl::Eq<Self, T::Expression>;
                fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                    use diesel::expression_methods::ExpressionMethods;
                    self.eq(__diesel_internal_rhs)
                }
            }
            impl
                diesel::query_source::AppearsInFromClause<diesel::query_builder::Only<super::table>>
                for title
            {
                type Count = diesel::query_source::Once;
            }
            impl diesel::SelectableExpression<diesel::query_builder::Only<super::table>> for title {}
            impl<TSM>
                diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for title
            where
                TSM: diesel::internal::table_macro::TablesampleMethod,
            {
                type Count = diesel::query_source::Once;
            }
            impl<TSM>
                diesel::SelectableExpression<diesel::query_builder::Tablesample<super::table, TSM>>
                for title
            where
                TSM: diesel::internal::table_macro::TablesampleMethod,
            {
            }
            impl diesel::expression::IsContainedInGroupBy<id> for title {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<title> for id {
                type Output = diesel::expression::is_contained_in_group_by::Yes;
            }
        }
    }
    #[allow(unused_imports)]
    const _: () = {
        use diesel;
        use diesel::deserialize::{self, FromStaticSqlRow, Queryable};
        use diesel::row::{Field as _, Row as _};
        use std::convert::TryInto;
        impl<__DB: diesel::backend::Backend, __ST0, __ST1> Queryable<(__ST0, __ST1), __DB> for Book
        where
            (i32, String): FromStaticSqlRow<(__ST0, __ST1), __DB>,
        {
            type Row = (i32, String);
            fn build(row: Self::Row) -> deserialize::Result<Self> {
                Ok(Self {
                    id: row.0.try_into()?,
                    title: row.1.try_into()?,
                })
            }
        }
    };
    #[allow(unused_imports)]
    const _: () = {
        use diesel;
        use diesel::associations::{HasTable, Identifiable};
        impl HasTable for Book {
            type Table = book_table::table;
            fn table() -> Self::Table {
                book_table::table
            }
        }
        impl<'ident> Identifiable for &'ident Book {
            type Id = (&'ident i32);
            fn id(self) -> Self::Id {
                (&self.id)
            }
        }
        impl<'ident> Identifiable for &'_ &'ident Book {
            type Id = (&'ident i32);
            fn id(self) -> Self::Id {
                (&self.id)
            }
        }
    };
    #[allow(unused_imports)]
    const _: () = {
        use diesel;
        use diesel::expression::Selectable;
        impl<__DB: diesel::backend::Backend> Selectable<__DB> for Book {
            type SelectExpression = (book_table::id, book_table::title);
            fn construct_selection() -> Self::SelectExpression {
                (book_table::id, book_table::title)
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for Book {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Book",
                "id",
                &self.id,
                "title",
                &&self.title,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Book {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Book {
        #[inline]
        fn eq(&self, other: &Book) -> bool {
            self.id == other.id && self.title == other.title
        }
    }
    #[allow(unused_imports)]
    const _: () = {
        use diesel;
        use diesel::insertable::Insertable;
        use diesel::internal::derives::insertable::UndecoratedInsertRecord;
        use diesel::prelude::*;
        #[allow(unused_qualifications)]
        impl Insertable<book_table::table> for Book {
            type Values = <(
                std::option::Option<diesel::dsl::Eq<book_table::id, i32>>,
                std::option::Option<diesel::dsl::Eq<book_table::title, String>>,
            ) as Insertable<book_table::table>>::Values;
            fn values(
                self,
            ) -> <(
                std::option::Option<diesel::dsl::Eq<book_table::id, i32>>,
                std::option::Option<diesel::dsl::Eq<book_table::title, String>>,
            ) as Insertable<book_table::table>>::Values {
                (
                    std::option::Option::Some(book_table::id.eq(self.id)),
                    std::option::Option::Some(book_table::title.eq(self.title)),
                )
                    .values()
            }
        }
        #[allow(unused_qualifications)]
        impl<'insert> Insertable<book_table::table> for &'insert Book {
            type Values = <(
                std::option::Option<diesel::dsl::Eq<book_table::id, &'insert i32>>,
                std::option::Option<diesel::dsl::Eq<book_table::title, &'insert String>>,
            ) as Insertable<book_table::table>>::Values;
            fn values(
                self,
            ) -> <(
                std::option::Option<diesel::dsl::Eq<book_table::id, &'insert i32>>,
                std::option::Option<diesel::dsl::Eq<book_table::title, &'insert String>>,
            ) as Insertable<book_table::table>>::Values {
                (
                    std::option::Option::Some(book_table::id.eq(&self.id)),
                    std::option::Option::Some(book_table::title.eq(&self.title)),
                )
                    .values()
            }
        }
        impl UndecoratedInsertRecord<book_table::table> for Book {}
    };
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
    #[diesel(table_name = page_table)]
    #[diesel(belongs_to(Book))]
    pub struct Page {
        pub id: i32,
        pub page_number: i32,
        pub content: String,
        pub book_id: i32,
    }
    pub enum PageTable {
        Id,
        PageNumber,
        Content,
        BookId,
        Table,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for PageTable {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    PageTable::Id => "Id",
                    PageTable::PageNumber => "PageNumber",
                    PageTable::Content => "Content",
                    PageTable::BookId => "BookId",
                    PageTable::Table => "Table",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for PageTable {
        #[inline]
        fn clone(&self) -> PageTable {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for PageTable {}
    impl sea_query::Iden for PageTable {
        fn prepare(&self, s: &mut dyn ::std::fmt::Write, q: sea_query::Quote) {
            s.write_fmt(format_args!("{0}", q.left())).unwrap();
            self.unquoted(s);
            s.write_fmt(format_args!("{0}", q.right())).unwrap();
        }
        fn unquoted(&self, s: &mut dyn ::std::fmt::Write) {
            match self {
                Self::Id => s.write_fmt(format_args!("{0}", "id")).unwrap(),
                Self::PageNumber => s.write_fmt(format_args!("{0}", "page_number")).unwrap(),
                Self::Content => s.write_fmt(format_args!("{0}", "content")).unwrap(),
                Self::BookId => s.write_fmt(format_args!("{0}", "book_id")).unwrap(),
                Self::Table => s.write_fmt(format_args!("{0}", "page_table")).unwrap(),
            };
        }
    }
    #[allow(unused_imports, dead_code, unreachable_pub, unused_qualifications)]
    pub mod page_table {
        pub use self::columns::*;
        use ::diesel;
        use diesel::sql_types::*;
        /// Re-exports all of the columns of this table, as well as the
        /// table struct renamed to the module name. This is meant to be
        /// glob imported for functions which only deal with one table.
        pub mod dsl {
            pub use super::columns::book_id;
            pub use super::columns::content;
            pub use super::columns::id;
            pub use super::columns::page_number;
            pub use super::table as page_table;
        }
        #[allow(non_upper_case_globals, dead_code)]
        /// A tuple of all of the columns on this table
        pub const all_columns: (id, page_number, content, book_id) =
            (id, page_number, content, book_id);
        #[allow(non_camel_case_types)]
        /// The actual table struct
        ///
        /// This is the type which provides the base methods of the query
        /// builder, such as `.select` and `.filter`.
        pub struct table;
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::fmt::Debug for table {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "table")
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::clone::Clone for table {
            #[inline]
            fn clone(&self) -> table {
                *self
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::marker::Copy for table {}
        #[allow(unused_imports)]
        const _: () = {
            use diesel;
            use diesel::query_builder::QueryId;
            #[allow(non_camel_case_types)]
            impl QueryId for table {
                type QueryId = table;
                const HAS_STATIC_QUERY_ID: bool = true;
            }
        };
        #[automatically_derived]
        #[allow(non_camel_case_types)]
        impl ::core::default::Default for table {
            #[inline]
            fn default() -> table {
                table {}
            }
        }
        impl table {
            #[allow(dead_code)]
            /// Represents `table_name.*`, which is sometimes necessary
            /// for efficient count queries. It cannot be used in place of
            /// `all_columns`
            pub fn star(&self) -> star {
                star
            }
        }
        /// The SQL type of all of the columns on this table
        pub type SqlType = (Integer, Integer, Text, Integer);
        /// Helper type for representing a boxed query from this table
        pub type BoxedQuery<'a, DB, ST = SqlType> =
            diesel::internal::table_macro::BoxedSelectStatement<
                'a,
                ST,
                diesel::internal::table_macro::FromClause<table>,
                DB,
            >;
        impl diesel::QuerySource for table {
            type FromClause = diesel::internal::table_macro::StaticQueryFragmentInstance<table>;
            type DefaultSelection = <Self as diesel::Table>::AllColumns;
            fn from_clause(&self) -> Self::FromClause {
                diesel::internal::table_macro::StaticQueryFragmentInstance::new()
            }
            fn default_selection(&self) -> Self::DefaultSelection {
                use diesel::Table;
                Self::all_columns()
            }
        }
        impl<DB> diesel::query_builder::QueryFragment<DB> for table
        where
            DB: diesel::backend::Backend,
            <table as diesel::internal::table_macro::StaticQueryFragment>::Component:
                diesel::query_builder::QueryFragment<DB>,
        {
            fn walk_ast<'b>(
                &'b self,
                __diesel_internal_pass: diesel::query_builder::AstPass<'_, 'b, DB>,
            ) -> diesel::result::QueryResult<()> {
                <table as diesel::internal::table_macro::StaticQueryFragment>::STATIC_COMPONENT
                    .walk_ast(__diesel_internal_pass)
            }
        }
        impl diesel::internal::table_macro::StaticQueryFragment for table {
            type Component = diesel::internal::table_macro::Identifier<'static>;
            const STATIC_COMPONENT: &'static Self::Component =
                &diesel::internal::table_macro::Identifier("page_table");
        }
        impl diesel::query_builder::AsQuery for table {
            type SqlType = SqlType;
            type Query = diesel::internal::table_macro::SelectStatement<
                diesel::internal::table_macro::FromClause<Self>,
            >;
            fn as_query(self) -> Self::Query {
                diesel::internal::table_macro::SelectStatement::simple(self)
            }
        }
        impl diesel::Table for table {
            type PrimaryKey = id;
            type AllColumns = (id, page_number, content, book_id);
            fn primary_key(&self) -> Self::PrimaryKey {
                id
            }
            fn all_columns() -> Self::AllColumns {
                (id, page_number, content, book_id)
            }
        }
        impl diesel::associations::HasTable for table {
            type Table = Self;
            fn table() -> Self::Table {
                table
            }
        }
        impl diesel::query_builder::IntoUpdateTarget for table {
            type WhereClause = <<Self as diesel::query_builder::AsQuery>::Query as diesel::query_builder::IntoUpdateTarget>::WhereClause;
            fn into_update_target(
                self,
            ) -> diesel::query_builder::UpdateTarget<Self::Table, Self::WhereClause> {
                use diesel::query_builder::AsQuery;
                let q: diesel::internal::table_macro::SelectStatement<
                    diesel::internal::table_macro::FromClause<table>,
                > = self.as_query();
                q.into_update_target()
            }
        }
        impl diesel::query_source::AppearsInFromClause<table> for table {
            type Count = diesel::query_source::Once;
        }
        impl<S> diesel::internal::table_macro::AliasAppearsInFromClause<S, table> for table
        where
            S: diesel::query_source::AliasSource<Target = table>,
        {
            type Count = diesel::query_source::Never;
        }
        impl<S1, S2> diesel::internal::table_macro::AliasAliasAppearsInFromClause<table, S2, S1> for table
        where
            S1: diesel::query_source::AliasSource<Target = table>,
            S2: diesel::query_source::AliasSource<Target = table>,
            S1: diesel::internal::table_macro::AliasAliasAppearsInFromClauseSameTable<S2, table>,
        {
            type Count =
                <S1 as diesel::internal::table_macro::AliasAliasAppearsInFromClauseSameTable<
                    S2,
                    table,
                >>::Count;
        }
        impl<S> diesel::query_source::AppearsInFromClause<diesel::query_source::Alias<S>> for table
        where
            S: diesel::query_source::AliasSource,
        {
            type Count = diesel::query_source::Never;
        }
        impl<S, C>
            diesel::internal::table_macro::FieldAliasMapperAssociatedTypesDisjointnessTrick<
                table,
                S,
                C,
            > for table
        where
            S: diesel::query_source::AliasSource<Target = table> + ::std::clone::Clone,
            C: diesel::query_source::Column<Table = table>,
        {
            type Out = diesel::query_source::AliasedField<S, C>;
            fn map(
                __diesel_internal_column: C,
                __diesel_internal_alias: &diesel::query_source::Alias<S>,
            ) -> Self::Out {
                __diesel_internal_alias.field(__diesel_internal_column)
            }
        }
        impl diesel::query_source::AppearsInFromClause<table>
            for diesel::internal::table_macro::NoFromClause
        {
            type Count = diesel::query_source::Never;
        }
        impl<Left, Right, Kind>
            diesel::JoinTo<diesel::internal::table_macro::Join<Left, Right, Kind>> for table
        where
            diesel::internal::table_macro::Join<Left, Right, Kind>: diesel::JoinTo<table>,
            Left: diesel::query_source::QuerySource,
            Right: diesel::query_source::QuerySource,
        {
            type FromClause = diesel::internal::table_macro::Join<Left, Right, Kind>;
            type OnClause = <diesel::internal::table_macro::Join<
                Left,
                Right,
                Kind,
            > as diesel::JoinTo<table>>::OnClause;
            fn join_target(
                __diesel_internal_rhs: diesel::internal::table_macro::Join<Left, Right, Kind>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, __diesel_internal_on_clause) =
                    diesel::internal::table_macro::Join::join_target(table);
                (__diesel_internal_rhs, __diesel_internal_on_clause)
            }
        }
        impl<Join, On> diesel::JoinTo<diesel::internal::table_macro::JoinOn<Join, On>> for table
        where
            diesel::internal::table_macro::JoinOn<Join, On>: diesel::JoinTo<table>,
        {
            type FromClause = diesel::internal::table_macro::JoinOn<Join, On>;
            type OnClause = <diesel::internal::table_macro::JoinOn<Join, On> as diesel::JoinTo<
                table,
            >>::OnClause;
            fn join_target(
                __diesel_internal_rhs: diesel::internal::table_macro::JoinOn<Join, On>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, __diesel_internal_on_clause) =
                    diesel::internal::table_macro::JoinOn::join_target(table);
                (__diesel_internal_rhs, __diesel_internal_on_clause)
            }
        }
        impl<F, S, D, W, O, L, Of, G>
            diesel::JoinTo<
                diesel::internal::table_macro::SelectStatement<
                    diesel::internal::table_macro::FromClause<F>,
                    S,
                    D,
                    W,
                    O,
                    L,
                    Of,
                    G,
                >,
            > for table
        where
            diesel::internal::table_macro::SelectStatement<
                diesel::internal::table_macro::FromClause<F>,
                S,
                D,
                W,
                O,
                L,
                Of,
                G,
            >: diesel::JoinTo<table>,
            F: diesel::query_source::QuerySource,
        {
            type FromClause = diesel::internal::table_macro::SelectStatement<
                diesel::internal::table_macro::FromClause<F>,
                S,
                D,
                W,
                O,
                L,
                Of,
                G,
            >;
            type OnClause = <diesel::internal::table_macro::SelectStatement<
                diesel::internal::table_macro::FromClause<F>,
                S,
                D,
                W,
                O,
                L,
                Of,
                G,
            > as diesel::JoinTo<table>>::OnClause;
            fn join_target(
                __diesel_internal_rhs: diesel::internal::table_macro::SelectStatement<
                    diesel::internal::table_macro::FromClause<F>,
                    S,
                    D,
                    W,
                    O,
                    L,
                    Of,
                    G,
                >,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, __diesel_internal_on_clause) =
                    diesel::internal::table_macro::SelectStatement::join_target(table);
                (__diesel_internal_rhs, __diesel_internal_on_clause)
            }
        }
        impl<'a, QS, ST, DB>
            diesel::JoinTo<
                diesel::internal::table_macro::BoxedSelectStatement<
                    'a,
                    diesel::internal::table_macro::FromClause<QS>,
                    ST,
                    DB,
                >,
            > for table
        where
            diesel::internal::table_macro::BoxedSelectStatement<
                'a,
                diesel::internal::table_macro::FromClause<QS>,
                ST,
                DB,
            >: diesel::JoinTo<table>,
            QS: diesel::query_source::QuerySource,
        {
            type FromClause = diesel::internal::table_macro::BoxedSelectStatement<
                'a,
                diesel::internal::table_macro::FromClause<QS>,
                ST,
                DB,
            >;
            type OnClause = <diesel::internal::table_macro::BoxedSelectStatement<
                'a,
                diesel::internal::table_macro::FromClause<QS>,
                ST,
                DB,
            > as diesel::JoinTo<table>>::OnClause;
            fn join_target(
                __diesel_internal_rhs: diesel::internal::table_macro::BoxedSelectStatement<
                    'a,
                    diesel::internal::table_macro::FromClause<QS>,
                    ST,
                    DB,
                >,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, __diesel_internal_on_clause) =
                    diesel::internal::table_macro::BoxedSelectStatement::join_target(table);
                (__diesel_internal_rhs, __diesel_internal_on_clause)
            }
        }
        impl<S> diesel::JoinTo<diesel::query_source::Alias<S>> for table
        where
            diesel::query_source::Alias<S>: diesel::JoinTo<table>,
        {
            type FromClause = diesel::query_source::Alias<S>;
            type OnClause = <diesel::query_source::Alias<S> as diesel::JoinTo<table>>::OnClause;
            fn join_target(
                __diesel_internal_rhs: diesel::query_source::Alias<S>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, __diesel_internal_on_clause) =
                    diesel::query_source::Alias::<S>::join_target(table);
                (__diesel_internal_rhs, __diesel_internal_on_clause)
            }
        }
        impl<T> diesel::insertable::Insertable<T> for table
        where
            <table as diesel::query_builder::AsQuery>::Query: diesel::insertable::Insertable<T>,
        {
            type Values = <<table as diesel::query_builder::AsQuery>::Query as diesel::insertable::Insertable<
                T,
            >>::Values;
            fn values(self) -> Self::Values {
                use diesel::query_builder::AsQuery;
                self.as_query().values()
            }
        }
        impl<'a, T> diesel::insertable::Insertable<T> for &'a table
        where
            table: diesel::insertable::Insertable<T>,
        {
            type Values = <table as diesel::insertable::Insertable<T>>::Values;
            fn values(self) -> Self::Values {
                (*self).values()
            }
        }
        impl<S> diesel::JoinTo<diesel::query_builder::Only<S>> for table
        where
            diesel::query_builder::Only<S>: diesel::JoinTo<table>,
        {
            type FromClause = diesel::query_builder::Only<S>;
            type OnClause = <diesel::query_builder::Only<S> as diesel::JoinTo<table>>::OnClause;
            fn join_target(
                __diesel_internal_rhs: diesel::query_builder::Only<S>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, __diesel_internal_on_clause) =
                    diesel::query_builder::Only::<S>::join_target(table);
                (__diesel_internal_rhs, __diesel_internal_on_clause)
            }
        }
        impl diesel::query_source::AppearsInFromClause<diesel::query_builder::Only<table>> for table {
            type Count = diesel::query_source::Once;
        }
        impl diesel::query_source::AppearsInFromClause<table> for diesel::query_builder::Only<table> {
            type Count = diesel::query_source::Once;
        }
        impl<S, TSM> diesel::JoinTo<diesel::query_builder::Tablesample<S, TSM>> for table
        where
            diesel::query_builder::Tablesample<S, TSM>: diesel::JoinTo<table>,
            TSM: diesel::internal::table_macro::TablesampleMethod,
        {
            type FromClause = diesel::query_builder::Tablesample<S, TSM>;
            type OnClause =
                <diesel::query_builder::Tablesample<S, TSM> as diesel::JoinTo<table>>::OnClause;
            fn join_target(
                __diesel_internal_rhs: diesel::query_builder::Tablesample<S, TSM>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, __diesel_internal_on_clause) =
                    diesel::query_builder::Tablesample::<S, TSM>::join_target(table);
                (__diesel_internal_rhs, __diesel_internal_on_clause)
            }
        }
        impl<TSM>
            diesel::query_source::AppearsInFromClause<
                diesel::query_builder::Tablesample<table, TSM>,
            > for table
        where
            TSM: diesel::internal::table_macro::TablesampleMethod,
        {
            type Count = diesel::query_source::Once;
        }
        impl<TSM> diesel::query_source::AppearsInFromClause<table>
            for diesel::query_builder::Tablesample<table, TSM>
        where
            TSM: diesel::internal::table_macro::TablesampleMethod,
        {
            type Count = diesel::query_source::Once;
        }
        /// Contains all of the columns of this table
        pub mod columns {
            use super::table;
            use ::diesel;
            use diesel::sql_types::*;
            #[allow(non_camel_case_types, dead_code)]
            /// Represents `table_name.*`, which is sometimes needed for
            /// efficient count queries. It cannot be used in place of
            /// `all_columns`, and has a `SqlType` of `()` to prevent it
            /// being used that way
            pub struct star;
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for star {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "star")
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for star {
                #[inline]
                fn clone(&self) -> star {
                    *self
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for star {}
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for star {
                    type QueryId = star;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            };
            impl<__GB> diesel::expression::ValidGrouping<__GB> for star
            where
                (id, page_number, content, book_id): diesel::expression::ValidGrouping<__GB>,
            {
                type IsAggregate =
                    <(id, page_number, content, book_id) as diesel::expression::ValidGrouping<
                        __GB,
                    >>::IsAggregate;
            }
            impl diesel::Expression for star {
                type SqlType = diesel::expression::expression_types::NotSelectable;
            }
            impl<DB: diesel::backend::Backend> diesel::query_builder::QueryFragment<DB> for star
            where
                <table as diesel::QuerySource>::FromClause:
                    diesel::query_builder::QueryFragment<DB>,
            {
                #[allow(non_snake_case)]
                fn walk_ast<'b>(
                    &'b self,
                    mut __diesel_internal_out: diesel::query_builder::AstPass<'_, 'b, DB>,
                ) -> diesel::result::QueryResult<()> {
                    use diesel::QuerySource;
                    if !__diesel_internal_out.should_skip_from() {
                        const FROM_CLAUSE:
                            diesel::internal::table_macro::StaticQueryFragmentInstance<table> =
                            diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                        FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                        __diesel_internal_out.push_sql(".");
                    }
                    __diesel_internal_out.push_sql("*");
                    Ok(())
                }
            }
            impl diesel::SelectableExpression<table> for star {}
            impl diesel::AppearsOnTable<table> for star {}
            #[allow(non_camel_case_types, dead_code)]
            pub struct id;
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for id {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "id")
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for id {
                #[inline]
                fn clone(&self) -> id {
                    *self
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for id {}
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for id {
                    type QueryId = id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            };
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for id {
                #[inline]
                fn default() -> id {
                    id {}
                }
            }
            impl diesel::expression::Expression for id {
                type SqlType = Integer;
            }
            impl<DB> diesel::query_builder::QueryFragment<DB> for id
            where
                DB: diesel::backend::Backend,
                diesel::internal::table_macro::StaticQueryFragmentInstance<table>:
                    diesel::query_builder::QueryFragment<DB>,
            {
                #[allow(non_snake_case)]
                fn walk_ast<'b>(
                    &'b self,
                    mut __diesel_internal_out: diesel::query_builder::AstPass<'_, 'b, DB>,
                ) -> diesel::result::QueryResult<()> {
                    if !__diesel_internal_out.should_skip_from() {
                        const FROM_CLAUSE:
                            diesel::internal::table_macro::StaticQueryFragmentInstance<table> =
                            diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                        FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                        __diesel_internal_out.push_sql(".");
                    }
                    __diesel_internal_out.push_identifier("id")
                }
            }
            impl diesel::SelectableExpression<super::table> for id {}
            impl<QS> diesel::AppearsOnTable<QS> for id where
                QS: diesel::query_source::AppearsInFromClause<
                    super::table,
                    Count = diesel::query_source::Once,
                >
            {
            }
            impl<Left, Right>
                diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for id
            where
                id: diesel::AppearsOnTable<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                >,
                Self: diesel::SelectableExpression<Left>,
                Right: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Never,
                    > + diesel::query_source::QuerySource,
                Left: diesel::query_source::QuerySource,
            {
            }
            impl<Left, Right>
                diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for id
            where
                id: diesel::AppearsOnTable<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                >,
                Left: diesel::query_source::AppearsInFromClause<super::table>
                    + diesel::query_source::QuerySource,
                Right: diesel::query_source::AppearsInFromClause<super::table>
                    + diesel::query_source::QuerySource,
                (Left::Count, Right::Count): diesel::internal::table_macro::Pick<Left, Right>,
                Self: diesel::SelectableExpression<
                    <(Left::Count, Right::Count) as diesel::internal::table_macro::Pick<
                        Left,
                        Right,
                    >>::Selection,
                >,
            {
            }
            impl<Join, On>
                diesel::SelectableExpression<diesel::internal::table_macro::JoinOn<Join, On>> for id
            where
                id: diesel::SelectableExpression<Join>
                    + diesel::AppearsOnTable<diesel::internal::table_macro::JoinOn<Join, On>>,
            {
            }
            impl<From>
                diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for id
            where
                From: diesel::query_source::QuerySource,
                id: diesel::SelectableExpression<From>
                    + diesel::AppearsOnTable<
                        diesel::internal::table_macro::SelectStatement<
                            diesel::internal::table_macro::FromClause<From>,
                        >,
                    >,
            {
            }
            impl<__GB> diesel::expression::ValidGrouping<__GB> for id
            where
                __GB: diesel::expression::IsContainedInGroupBy<
                    id,
                    Output = diesel::expression::is_contained_in_group_by::Yes,
                >,
            {
                type IsAggregate = diesel::expression::is_aggregate::Yes;
            }
            impl diesel::expression::ValidGrouping<()> for id {
                type IsAggregate = diesel::expression::is_aggregate::No;
            }
            impl diesel::expression::IsContainedInGroupBy<id> for id {
                type Output = diesel::expression::is_contained_in_group_by::Yes;
            }
            impl diesel::query_source::Column for id {
                type Table = super::table;
                const NAME: &'static str = "id";
            }
            impl<T> diesel::EqAll<T> for id
            where
                T: diesel::expression::AsExpression<Integer>,
                diesel::dsl::Eq<id, T::Expression>:
                    diesel::Expression<SqlType = diesel::sql_types::Bool>,
            {
                type Output = diesel::dsl::Eq<Self, T::Expression>;
                fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                    use diesel::expression_methods::ExpressionMethods;
                    self.eq(__diesel_internal_rhs)
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for id
            where
                Rhs: diesel::expression::AsExpression<
                    <<id as diesel::Expression>::SqlType as diesel::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = diesel::internal::table_macro::ops::Add<Self, Rhs::Expression>;
                fn add(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                    diesel::internal::table_macro::ops::Add::new(
                        self,
                        __diesel_internal_rhs.as_expression(),
                    )
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for id
            where
                Rhs: diesel::expression::AsExpression<
                    <<id as diesel::Expression>::SqlType as diesel::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = diesel::internal::table_macro::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                    diesel::internal::table_macro::ops::Sub::new(
                        self,
                        __diesel_internal_rhs.as_expression(),
                    )
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for id
            where
                Rhs: diesel::expression::AsExpression<
                    <<id as diesel::Expression>::SqlType as diesel::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = diesel::internal::table_macro::ops::Div<Self, Rhs::Expression>;
                fn div(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                    diesel::internal::table_macro::ops::Div::new(
                        self,
                        __diesel_internal_rhs.as_expression(),
                    )
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for id
            where
                Rhs: diesel::expression::AsExpression<
                    <<id as diesel::Expression>::SqlType as diesel::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = diesel::internal::table_macro::ops::Mul<Self, Rhs::Expression>;
                fn mul(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                    diesel::internal::table_macro::ops::Mul::new(
                        self,
                        __diesel_internal_rhs.as_expression(),
                    )
                }
            }
            impl
                diesel::query_source::AppearsInFromClause<diesel::query_builder::Only<super::table>>
                for id
            {
                type Count = diesel::query_source::Once;
            }
            impl diesel::SelectableExpression<diesel::query_builder::Only<super::table>> for id {}
            impl<TSM>
                diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for id
            where
                TSM: diesel::internal::table_macro::TablesampleMethod,
            {
                type Count = diesel::query_source::Once;
            }
            impl<TSM>
                diesel::SelectableExpression<diesel::query_builder::Tablesample<super::table, TSM>>
                for id
            where
                TSM: diesel::internal::table_macro::TablesampleMethod,
            {
            }
            #[allow(non_camel_case_types, dead_code)]
            pub struct page_number;
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for page_number {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "page_number")
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for page_number {
                #[inline]
                fn clone(&self) -> page_number {
                    *self
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for page_number {}
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for page_number {
                    type QueryId = page_number;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            };
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for page_number {
                #[inline]
                fn default() -> page_number {
                    page_number {}
                }
            }
            impl diesel::expression::Expression for page_number {
                type SqlType = Integer;
            }
            impl<DB> diesel::query_builder::QueryFragment<DB> for page_number
            where
                DB: diesel::backend::Backend,
                diesel::internal::table_macro::StaticQueryFragmentInstance<table>:
                    diesel::query_builder::QueryFragment<DB>,
            {
                #[allow(non_snake_case)]
                fn walk_ast<'b>(
                    &'b self,
                    mut __diesel_internal_out: diesel::query_builder::AstPass<'_, 'b, DB>,
                ) -> diesel::result::QueryResult<()> {
                    if !__diesel_internal_out.should_skip_from() {
                        const FROM_CLAUSE:
                            diesel::internal::table_macro::StaticQueryFragmentInstance<table> =
                            diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                        FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                        __diesel_internal_out.push_sql(".");
                    }
                    __diesel_internal_out.push_identifier("page_number")
                }
            }
            impl diesel::SelectableExpression<super::table> for page_number {}
            impl<QS> diesel::AppearsOnTable<QS> for page_number where
                QS: diesel::query_source::AppearsInFromClause<
                    super::table,
                    Count = diesel::query_source::Once,
                >
            {
            }
            impl<Left, Right>
                diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for page_number
            where
                page_number: diesel::AppearsOnTable<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                >,
                Self: diesel::SelectableExpression<Left>,
                Right: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Never,
                    > + diesel::query_source::QuerySource,
                Left: diesel::query_source::QuerySource,
            {
            }
            impl<Left, Right>
                diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for page_number
            where
                page_number: diesel::AppearsOnTable<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                >,
                Left: diesel::query_source::AppearsInFromClause<super::table>
                    + diesel::query_source::QuerySource,
                Right: diesel::query_source::AppearsInFromClause<super::table>
                    + diesel::query_source::QuerySource,
                (Left::Count, Right::Count): diesel::internal::table_macro::Pick<Left, Right>,
                Self: diesel::SelectableExpression<
                    <(Left::Count, Right::Count) as diesel::internal::table_macro::Pick<
                        Left,
                        Right,
                    >>::Selection,
                >,
            {
            }
            impl<Join, On>
                diesel::SelectableExpression<diesel::internal::table_macro::JoinOn<Join, On>>
                for page_number
            where
                page_number: diesel::SelectableExpression<Join>
                    + diesel::AppearsOnTable<diesel::internal::table_macro::JoinOn<Join, On>>,
            {
            }
            impl<From>
                diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for page_number
            where
                From: diesel::query_source::QuerySource,
                page_number: diesel::SelectableExpression<From>
                    + diesel::AppearsOnTable<
                        diesel::internal::table_macro::SelectStatement<
                            diesel::internal::table_macro::FromClause<From>,
                        >,
                    >,
            {
            }
            impl<__GB> diesel::expression::ValidGrouping<__GB> for page_number
            where
                __GB: diesel::expression::IsContainedInGroupBy<
                    page_number,
                    Output = diesel::expression::is_contained_in_group_by::Yes,
                >,
            {
                type IsAggregate = diesel::expression::is_aggregate::Yes;
            }
            impl diesel::expression::ValidGrouping<()> for page_number {
                type IsAggregate = diesel::expression::is_aggregate::No;
            }
            impl diesel::expression::IsContainedInGroupBy<page_number> for page_number {
                type Output = diesel::expression::is_contained_in_group_by::Yes;
            }
            impl diesel::query_source::Column for page_number {
                type Table = super::table;
                const NAME: &'static str = "page_number";
            }
            impl<T> diesel::EqAll<T> for page_number
            where
                T: diesel::expression::AsExpression<Integer>,
                diesel::dsl::Eq<page_number, T::Expression>:
                    diesel::Expression<SqlType = diesel::sql_types::Bool>,
            {
                type Output = diesel::dsl::Eq<Self, T::Expression>;
                fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                    use diesel::expression_methods::ExpressionMethods;
                    self.eq(__diesel_internal_rhs)
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for page_number
            where
                Rhs: diesel::expression::AsExpression<
                    <<page_number as diesel::Expression>::SqlType as diesel::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = diesel::internal::table_macro::ops::Add<
                    Self,
                    Rhs::Expression,
                >;
                fn add(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                    diesel::internal::table_macro::ops::Add::new(
                        self,
                        __diesel_internal_rhs.as_expression(),
                    )
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for page_number
            where
                Rhs: diesel::expression::AsExpression<
                    <<page_number as diesel::Expression>::SqlType as diesel::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = diesel::internal::table_macro::ops::Sub<
                    Self,
                    Rhs::Expression,
                >;
                fn sub(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                    diesel::internal::table_macro::ops::Sub::new(
                        self,
                        __diesel_internal_rhs.as_expression(),
                    )
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for page_number
            where
                Rhs: diesel::expression::AsExpression<
                    <<page_number as diesel::Expression>::SqlType as diesel::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = diesel::internal::table_macro::ops::Div<
                    Self,
                    Rhs::Expression,
                >;
                fn div(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                    diesel::internal::table_macro::ops::Div::new(
                        self,
                        __diesel_internal_rhs.as_expression(),
                    )
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for page_number
            where
                Rhs: diesel::expression::AsExpression<
                    <<page_number as diesel::Expression>::SqlType as diesel::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = diesel::internal::table_macro::ops::Mul<
                    Self,
                    Rhs::Expression,
                >;
                fn mul(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                    diesel::internal::table_macro::ops::Mul::new(
                        self,
                        __diesel_internal_rhs.as_expression(),
                    )
                }
            }
            impl
                diesel::query_source::AppearsInFromClause<diesel::query_builder::Only<super::table>>
                for page_number
            {
                type Count = diesel::query_source::Once;
            }
            impl diesel::SelectableExpression<diesel::query_builder::Only<super::table>> for page_number {}
            impl<TSM>
                diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for page_number
            where
                TSM: diesel::internal::table_macro::TablesampleMethod,
            {
                type Count = diesel::query_source::Once;
            }
            impl<TSM>
                diesel::SelectableExpression<diesel::query_builder::Tablesample<super::table, TSM>>
                for page_number
            where
                TSM: diesel::internal::table_macro::TablesampleMethod,
            {
            }
            #[allow(non_camel_case_types, dead_code)]
            pub struct content;
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for content {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "content")
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for content {
                #[inline]
                fn clone(&self) -> content {
                    *self
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for content {}
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for content {
                    type QueryId = content;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            };
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for content {
                #[inline]
                fn default() -> content {
                    content {}
                }
            }
            impl diesel::expression::Expression for content {
                type SqlType = Text;
            }
            impl<DB> diesel::query_builder::QueryFragment<DB> for content
            where
                DB: diesel::backend::Backend,
                diesel::internal::table_macro::StaticQueryFragmentInstance<table>:
                    diesel::query_builder::QueryFragment<DB>,
            {
                #[allow(non_snake_case)]
                fn walk_ast<'b>(
                    &'b self,
                    mut __diesel_internal_out: diesel::query_builder::AstPass<'_, 'b, DB>,
                ) -> diesel::result::QueryResult<()> {
                    if !__diesel_internal_out.should_skip_from() {
                        const FROM_CLAUSE:
                            diesel::internal::table_macro::StaticQueryFragmentInstance<table> =
                            diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                        FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                        __diesel_internal_out.push_sql(".");
                    }
                    __diesel_internal_out.push_identifier("content")
                }
            }
            impl diesel::SelectableExpression<super::table> for content {}
            impl<QS> diesel::AppearsOnTable<QS> for content where
                QS: diesel::query_source::AppearsInFromClause<
                    super::table,
                    Count = diesel::query_source::Once,
                >
            {
            }
            impl<Left, Right>
                diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for content
            where
                content: diesel::AppearsOnTable<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                >,
                Self: diesel::SelectableExpression<Left>,
                Right: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Never,
                    > + diesel::query_source::QuerySource,
                Left: diesel::query_source::QuerySource,
            {
            }
            impl<Left, Right>
                diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for content
            where
                content: diesel::AppearsOnTable<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                >,
                Left: diesel::query_source::AppearsInFromClause<super::table>
                    + diesel::query_source::QuerySource,
                Right: diesel::query_source::AppearsInFromClause<super::table>
                    + diesel::query_source::QuerySource,
                (Left::Count, Right::Count): diesel::internal::table_macro::Pick<Left, Right>,
                Self: diesel::SelectableExpression<
                    <(Left::Count, Right::Count) as diesel::internal::table_macro::Pick<
                        Left,
                        Right,
                    >>::Selection,
                >,
            {
            }
            impl<Join, On>
                diesel::SelectableExpression<diesel::internal::table_macro::JoinOn<Join, On>>
                for content
            where
                content: diesel::SelectableExpression<Join>
                    + diesel::AppearsOnTable<diesel::internal::table_macro::JoinOn<Join, On>>,
            {
            }
            impl<From>
                diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for content
            where
                From: diesel::query_source::QuerySource,
                content: diesel::SelectableExpression<From>
                    + diesel::AppearsOnTable<
                        diesel::internal::table_macro::SelectStatement<
                            diesel::internal::table_macro::FromClause<From>,
                        >,
                    >,
            {
            }
            impl<__GB> diesel::expression::ValidGrouping<__GB> for content
            where
                __GB: diesel::expression::IsContainedInGroupBy<
                    content,
                    Output = diesel::expression::is_contained_in_group_by::Yes,
                >,
            {
                type IsAggregate = diesel::expression::is_aggregate::Yes;
            }
            impl diesel::expression::ValidGrouping<()> for content {
                type IsAggregate = diesel::expression::is_aggregate::No;
            }
            impl diesel::expression::IsContainedInGroupBy<content> for content {
                type Output = diesel::expression::is_contained_in_group_by::Yes;
            }
            impl diesel::query_source::Column for content {
                type Table = super::table;
                const NAME: &'static str = "content";
            }
            impl<T> diesel::EqAll<T> for content
            where
                T: diesel::expression::AsExpression<Text>,
                diesel::dsl::Eq<content, T::Expression>:
                    diesel::Expression<SqlType = diesel::sql_types::Bool>,
            {
                type Output = diesel::dsl::Eq<Self, T::Expression>;
                fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                    use diesel::expression_methods::ExpressionMethods;
                    self.eq(__diesel_internal_rhs)
                }
            }
            impl
                diesel::query_source::AppearsInFromClause<diesel::query_builder::Only<super::table>>
                for content
            {
                type Count = diesel::query_source::Once;
            }
            impl diesel::SelectableExpression<diesel::query_builder::Only<super::table>> for content {}
            impl<TSM>
                diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for content
            where
                TSM: diesel::internal::table_macro::TablesampleMethod,
            {
                type Count = diesel::query_source::Once;
            }
            impl<TSM>
                diesel::SelectableExpression<diesel::query_builder::Tablesample<super::table, TSM>>
                for content
            where
                TSM: diesel::internal::table_macro::TablesampleMethod,
            {
            }
            #[allow(non_camel_case_types, dead_code)]
            pub struct book_id;
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for book_id {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "book_id")
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for book_id {
                #[inline]
                fn clone(&self) -> book_id {
                    *self
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for book_id {}
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for book_id {
                    type QueryId = book_id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            };
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for book_id {
                #[inline]
                fn default() -> book_id {
                    book_id {}
                }
            }
            impl diesel::expression::Expression for book_id {
                type SqlType = Integer;
            }
            impl<DB> diesel::query_builder::QueryFragment<DB> for book_id
            where
                DB: diesel::backend::Backend,
                diesel::internal::table_macro::StaticQueryFragmentInstance<table>:
                    diesel::query_builder::QueryFragment<DB>,
            {
                #[allow(non_snake_case)]
                fn walk_ast<'b>(
                    &'b self,
                    mut __diesel_internal_out: diesel::query_builder::AstPass<'_, 'b, DB>,
                ) -> diesel::result::QueryResult<()> {
                    if !__diesel_internal_out.should_skip_from() {
                        const FROM_CLAUSE:
                            diesel::internal::table_macro::StaticQueryFragmentInstance<table> =
                            diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                        FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                        __diesel_internal_out.push_sql(".");
                    }
                    __diesel_internal_out.push_identifier("book_id")
                }
            }
            impl diesel::SelectableExpression<super::table> for book_id {}
            impl<QS> diesel::AppearsOnTable<QS> for book_id where
                QS: diesel::query_source::AppearsInFromClause<
                    super::table,
                    Count = diesel::query_source::Once,
                >
            {
            }
            impl<Left, Right>
                diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                > for book_id
            where
                book_id: diesel::AppearsOnTable<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::LeftOuter,
                    >,
                >,
                Self: diesel::SelectableExpression<Left>,
                Right: diesel::query_source::AppearsInFromClause<
                        super::table,
                        Count = diesel::query_source::Never,
                    > + diesel::query_source::QuerySource,
                Left: diesel::query_source::QuerySource,
            {
            }
            impl<Left, Right>
                diesel::SelectableExpression<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                > for book_id
            where
                book_id: diesel::AppearsOnTable<
                    diesel::internal::table_macro::Join<
                        Left,
                        Right,
                        diesel::internal::table_macro::Inner,
                    >,
                >,
                Left: diesel::query_source::AppearsInFromClause<super::table>
                    + diesel::query_source::QuerySource,
                Right: diesel::query_source::AppearsInFromClause<super::table>
                    + diesel::query_source::QuerySource,
                (Left::Count, Right::Count): diesel::internal::table_macro::Pick<Left, Right>,
                Self: diesel::SelectableExpression<
                    <(Left::Count, Right::Count) as diesel::internal::table_macro::Pick<
                        Left,
                        Right,
                    >>::Selection,
                >,
            {
            }
            impl<Join, On>
                diesel::SelectableExpression<diesel::internal::table_macro::JoinOn<Join, On>>
                for book_id
            where
                book_id: diesel::SelectableExpression<Join>
                    + diesel::AppearsOnTable<diesel::internal::table_macro::JoinOn<Join, On>>,
            {
            }
            impl<From>
                diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for book_id
            where
                From: diesel::query_source::QuerySource,
                book_id: diesel::SelectableExpression<From>
                    + diesel::AppearsOnTable<
                        diesel::internal::table_macro::SelectStatement<
                            diesel::internal::table_macro::FromClause<From>,
                        >,
                    >,
            {
            }
            impl<__GB> diesel::expression::ValidGrouping<__GB> for book_id
            where
                __GB: diesel::expression::IsContainedInGroupBy<
                    book_id,
                    Output = diesel::expression::is_contained_in_group_by::Yes,
                >,
            {
                type IsAggregate = diesel::expression::is_aggregate::Yes;
            }
            impl diesel::expression::ValidGrouping<()> for book_id {
                type IsAggregate = diesel::expression::is_aggregate::No;
            }
            impl diesel::expression::IsContainedInGroupBy<book_id> for book_id {
                type Output = diesel::expression::is_contained_in_group_by::Yes;
            }
            impl diesel::query_source::Column for book_id {
                type Table = super::table;
                const NAME: &'static str = "book_id";
            }
            impl<T> diesel::EqAll<T> for book_id
            where
                T: diesel::expression::AsExpression<Integer>,
                diesel::dsl::Eq<book_id, T::Expression>:
                    diesel::Expression<SqlType = diesel::sql_types::Bool>,
            {
                type Output = diesel::dsl::Eq<Self, T::Expression>;
                fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                    use diesel::expression_methods::ExpressionMethods;
                    self.eq(__diesel_internal_rhs)
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for book_id
            where
                Rhs: diesel::expression::AsExpression<
                    <<book_id as diesel::Expression>::SqlType as diesel::sql_types::ops::Add>::Rhs,
                >,
            {
                type Output = diesel::internal::table_macro::ops::Add<Self, Rhs::Expression>;
                fn add(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                    diesel::internal::table_macro::ops::Add::new(
                        self,
                        __diesel_internal_rhs.as_expression(),
                    )
                }
            }
            impl<Rhs> ::std::ops::Sub<Rhs> for book_id
            where
                Rhs: diesel::expression::AsExpression<
                    <<book_id as diesel::Expression>::SqlType as diesel::sql_types::ops::Sub>::Rhs,
                >,
            {
                type Output = diesel::internal::table_macro::ops::Sub<Self, Rhs::Expression>;
                fn sub(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                    diesel::internal::table_macro::ops::Sub::new(
                        self,
                        __diesel_internal_rhs.as_expression(),
                    )
                }
            }
            impl<Rhs> ::std::ops::Div<Rhs> for book_id
            where
                Rhs: diesel::expression::AsExpression<
                    <<book_id as diesel::Expression>::SqlType as diesel::sql_types::ops::Div>::Rhs,
                >,
            {
                type Output = diesel::internal::table_macro::ops::Div<Self, Rhs::Expression>;
                fn div(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                    diesel::internal::table_macro::ops::Div::new(
                        self,
                        __diesel_internal_rhs.as_expression(),
                    )
                }
            }
            impl<Rhs> ::std::ops::Mul<Rhs> for book_id
            where
                Rhs: diesel::expression::AsExpression<
                    <<book_id as diesel::Expression>::SqlType as diesel::sql_types::ops::Mul>::Rhs,
                >,
            {
                type Output = diesel::internal::table_macro::ops::Mul<Self, Rhs::Expression>;
                fn mul(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                    diesel::internal::table_macro::ops::Mul::new(
                        self,
                        __diesel_internal_rhs.as_expression(),
                    )
                }
            }
            impl
                diesel::query_source::AppearsInFromClause<diesel::query_builder::Only<super::table>>
                for book_id
            {
                type Count = diesel::query_source::Once;
            }
            impl diesel::SelectableExpression<diesel::query_builder::Only<super::table>> for book_id {}
            impl<TSM>
                diesel::query_source::AppearsInFromClause<
                    diesel::query_builder::Tablesample<super::table, TSM>,
                > for book_id
            where
                TSM: diesel::internal::table_macro::TablesampleMethod,
            {
                type Count = diesel::query_source::Once;
            }
            impl<TSM>
                diesel::SelectableExpression<diesel::query_builder::Tablesample<super::table, TSM>>
                for book_id
            where
                TSM: diesel::internal::table_macro::TablesampleMethod,
            {
            }
            impl diesel::expression::IsContainedInGroupBy<id> for page_number {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<page_number> for id {
                type Output = diesel::expression::is_contained_in_group_by::Yes;
            }
            impl diesel::expression::IsContainedInGroupBy<id> for content {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<content> for id {
                type Output = diesel::expression::is_contained_in_group_by::Yes;
            }
            impl diesel::expression::IsContainedInGroupBy<id> for book_id {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<book_id> for id {
                type Output = diesel::expression::is_contained_in_group_by::Yes;
            }
            impl diesel::expression::IsContainedInGroupBy<page_number> for content {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<content> for page_number {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<page_number> for book_id {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<book_id> for page_number {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<content> for book_id {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<book_id> for content {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
        }
    }
    #[allow(unused_imports)]
    const _: () = {
        use diesel;
        use diesel::deserialize::{self, FromStaticSqlRow, Queryable};
        use diesel::row::{Field as _, Row as _};
        use std::convert::TryInto;
        impl<__DB: diesel::backend::Backend, __ST0, __ST1, __ST2, __ST3>
            Queryable<(__ST0, __ST1, __ST2, __ST3), __DB> for Page
        where
            (i32, i32, String, i32): FromStaticSqlRow<(__ST0, __ST1, __ST2, __ST3), __DB>,
        {
            type Row = (i32, i32, String, i32);
            fn build(row: Self::Row) -> deserialize::Result<Self> {
                Ok(Self {
                    id: row.0.try_into()?,
                    page_number: row.1.try_into()?,
                    content: row.2.try_into()?,
                    book_id: row.3.try_into()?,
                })
            }
        }
    };
    #[allow(unused_imports)]
    const _: () = {
        use diesel;
        use diesel::associations::{HasTable, Identifiable};
        impl HasTable for Page {
            type Table = page_table::table;
            fn table() -> Self::Table {
                page_table::table
            }
        }
        impl<'ident> Identifiable for &'ident Page {
            type Id = (&'ident i32);
            fn id(self) -> Self::Id {
                (&self.id)
            }
        }
        impl<'ident> Identifiable for &'_ &'ident Page {
            type Id = (&'ident i32);
            fn id(self) -> Self::Id {
                (&self.id)
            }
        }
    };
    #[allow(unused_imports)]
    const _: () = {
        use diesel;
        impl<__FK> diesel::associations::BelongsTo<Book> for Page
        where
            __FK: std::hash::Hash + std::cmp::Eq,
            for<'__a> &'__a i32: std::convert::Into<::std::option::Option<&'__a __FK>>,
            for<'__a> &'__a Book: diesel::associations::Identifiable<Id = &'__a __FK>,
        {
            type ForeignKey = __FK;
            type ForeignKeyColumn = page_table::book_id;
            fn foreign_key(&self) -> std::option::Option<&Self::ForeignKey> {
                std::convert::Into::into(&self.book_id)
            }
            fn foreign_key_column() -> Self::ForeignKeyColumn {
                page_table::book_id
            }
        }
        impl<__FK> diesel::associations::BelongsTo<&'_ Book> for Page
        where
            __FK: std::hash::Hash + std::cmp::Eq,
            for<'__a> &'__a i32: std::convert::Into<::std::option::Option<&'__a __FK>>,
            for<'__a> &'__a Book: diesel::associations::Identifiable<Id = &'__a __FK>,
        {
            type ForeignKey = __FK;
            type ForeignKeyColumn = page_table::book_id;
            fn foreign_key(&self) -> std::option::Option<&Self::ForeignKey> {
                std::convert::Into::into(&self.book_id)
            }
            fn foreign_key_column() -> Self::ForeignKeyColumn {
                page_table::book_id
            }
        }
    };
    #[allow(unused_imports)]
    const _: () = {
        use diesel;
        use diesel::expression::Selectable;
        impl<__DB: diesel::backend::Backend> Selectable<__DB> for Page {
            type SelectExpression = (
                page_table::id,
                page_table::page_number,
                page_table::content,
                page_table::book_id,
            );
            fn construct_selection() -> Self::SelectExpression {
                (
                    page_table::id,
                    page_table::page_number,
                    page_table::content,
                    page_table::book_id,
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for Page {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "Page",
                "id",
                &self.id,
                "page_number",
                &self.page_number,
                "content",
                &self.content,
                "book_id",
                &&self.book_id,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Page {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Page {
        #[inline]
        fn eq(&self, other: &Page) -> bool {
            self.id == other.id
                && self.page_number == other.page_number
                && self.content == other.content
                && self.book_id == other.book_id
        }
    }
    #[allow(unused_imports)]
    const _: () = {
        use diesel;
        use diesel::insertable::Insertable;
        use diesel::internal::derives::insertable::UndecoratedInsertRecord;
        use diesel::prelude::*;
        #[allow(unused_qualifications)]
        impl Insertable<page_table::table> for Page {
            type Values = <(
                std::option::Option<diesel::dsl::Eq<page_table::id, i32>>,
                std::option::Option<diesel::dsl::Eq<page_table::page_number, i32>>,
                std::option::Option<diesel::dsl::Eq<page_table::content, String>>,
                std::option::Option<diesel::dsl::Eq<page_table::book_id, i32>>,
            ) as Insertable<page_table::table>>::Values;
            fn values(
                self,
            ) -> <(
                std::option::Option<diesel::dsl::Eq<page_table::id, i32>>,
                std::option::Option<diesel::dsl::Eq<page_table::page_number, i32>>,
                std::option::Option<diesel::dsl::Eq<page_table::content, String>>,
                std::option::Option<diesel::dsl::Eq<page_table::book_id, i32>>,
            ) as Insertable<page_table::table>>::Values {
                (
                    std::option::Option::Some(page_table::id.eq(self.id)),
                    std::option::Option::Some(page_table::page_number.eq(self.page_number)),
                    std::option::Option::Some(page_table::content.eq(self.content)),
                    std::option::Option::Some(page_table::book_id.eq(self.book_id)),
                )
                    .values()
            }
        }
        #[allow(unused_qualifications)]
        impl<'insert> Insertable<page_table::table> for &'insert Page {
            type Values = <(
                std::option::Option<diesel::dsl::Eq<page_table::id, &'insert i32>>,
                std::option::Option<diesel::dsl::Eq<page_table::page_number, &'insert i32>>,
                std::option::Option<diesel::dsl::Eq<page_table::content, &'insert String>>,
                std::option::Option<diesel::dsl::Eq<page_table::book_id, &'insert i32>>,
            ) as Insertable<page_table::table>>::Values;
            fn values(
                self,
            ) -> <(
                std::option::Option<diesel::dsl::Eq<page_table::id, &'insert i32>>,
                std::option::Option<diesel::dsl::Eq<page_table::page_number, &'insert i32>>,
                std::option::Option<diesel::dsl::Eq<page_table::content, &'insert String>>,
                std::option::Option<diesel::dsl::Eq<page_table::book_id, &'insert i32>>,
            ) as Insertable<page_table::table>>::Values {
                (
                    std::option::Option::Some(page_table::id.eq(&self.id)),
                    std::option::Option::Some(page_table::page_number.eq(&self.page_number)),
                    std::option::Option::Some(page_table::content.eq(&self.content)),
                    std::option::Option::Some(page_table::book_id.eq(&self.book_id)),
                )
                    .values()
            }
        }
        impl UndecoratedInsertRecord<page_table::table> for Page {}
    };
    impl ::diesel::JoinTo<book_table::table> for page_table::table {
        type FromClause = book_table::table;
        type OnClause = ::diesel::dsl::Eq<
            ::diesel::internal::table_macro::NullableExpression<page_table::book_id>,
            ::diesel::internal::table_macro::NullableExpression<
                <book_table::table as ::diesel::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: book_table::table) -> (Self::FromClause, Self::OnClause) {
            use ::diesel::{ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                page_table::book_id.nullable().eq(
                    <book_table::table as ::diesel::query_source::Table>::primary_key(
                        &book_table::table,
                    )
                    .nullable(),
                ),
            )
        }
    }
    impl ::diesel::JoinTo<page_table::table> for book_table::table {
        type FromClause = page_table::table;
        type OnClause = ::diesel::dsl::Eq<
            ::diesel::internal::table_macro::NullableExpression<page_table::book_id>,
            ::diesel::internal::table_macro::NullableExpression<
                <book_table::table as ::diesel::query_source::Table>::PrimaryKey,
            >,
        >;
        fn join_target(rhs: page_table::table) -> (Self::FromClause, Self::OnClause) {
            use ::diesel::{ExpressionMethods, NullableExpressionMethods};
            (
                rhs,
                page_table::book_id.nullable().eq(
                    <book_table::table as ::diesel::query_source::Table>::primary_key(
                        &book_table::table,
                    )
                    .nullable(),
                ),
            )
        }
    }
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
    impl ::diesel::query_source::TableNotEqual<book_table::table> for page_table::table {}
    impl ::diesel::query_source::TableNotEqual<page_table::table> for book_table::table {}
    impl ::diesel::query_source::TableNotEqual<book_table::table>
        for ::diesel::query_builder::Only<page_table::table>
    {
    }
    impl ::diesel::query_source::TableNotEqual<page_table::table>
        for ::diesel::query_builder::Only<book_table::table>
    {
    }
    impl ::diesel::query_source::TableNotEqual<::diesel::query_builder::Only<book_table::table>>
        for page_table::table
    {
    }
    impl ::diesel::query_source::TableNotEqual<::diesel::query_builder::Only<page_table::table>>
        for book_table::table
    {
    }
    impl<TSM> ::diesel::query_source::TableNotEqual<book_table::table>
        for ::diesel::query_builder::Tablesample<page_table::table, TSM>
    where
        TSM: ::diesel::internal::table_macro::TablesampleMethod,
    {
    }
    impl<TSM> ::diesel::query_source::TableNotEqual<page_table::table>
        for ::diesel::query_builder::Tablesample<book_table::table, TSM>
    where
        TSM: ::diesel::internal::table_macro::TablesampleMethod,
    {
    }
    impl<TSM>
        ::diesel::query_source::TableNotEqual<
            ::diesel::query_builder::Tablesample<book_table::table, TSM>,
        > for page_table::table
    where
        TSM: ::diesel::internal::table_macro::TablesampleMethod,
    {
    }
    impl<TSM>
        ::diesel::query_source::TableNotEqual<
            ::diesel::query_builder::Tablesample<page_table::table, TSM>,
        > for book_table::table
    where
        TSM: ::diesel::internal::table_macro::TablesampleMethod,
    {
    }
}
