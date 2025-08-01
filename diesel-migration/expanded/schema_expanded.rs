pub mod schema {
    #[allow(unused_imports, dead_code, unreachable_pub, unused_qualifications)]
    pub mod posts {
        pub use self::columns::*;
        use ::diesel;
        use diesel::sql_types::*;
        /// Re-exports all of the columns of this table, as well as the
        /// table struct renamed to the module name. This is meant to be
        /// glob imported for functions which only deal with one table.
        pub mod dsl {
            pub use super::columns::body;
            pub use super::columns::id;
            pub use super::columns::published;
            pub use super::columns::title;
            pub use super::table as posts;
        }
        #[allow(non_upper_case_globals, dead_code)]
        /// A tuple of all of the columns on this table
        pub const all_columns: (id, title, body, published) = (id, title, body, published);
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
            #[allow(non_camel_case_types)]
            impl diesel::query_builder::QueryId for table {
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
        pub type SqlType = (Integer, Text, Text, Bool);
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
                &diesel::internal::table_macro::Identifier("posts");
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
            type AllColumns = (id, title, body, published);
            fn primary_key(&self) -> Self::PrimaryKey {
                id
            }
            fn all_columns() -> Self::AllColumns {
                (id, title, body, published)
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
                #[allow(non_camel_case_types)]
                impl diesel::query_builder::QueryId for star {
                    type QueryId = star;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            };
            impl<__GB> diesel::expression::ValidGrouping<__GB> for star
            where
                (id, title, body, published): diesel::expression::ValidGrouping<__GB>,
            {
                type IsAggregate = <(
                    id,
                    title,
                    body,
                    published,
                ) as diesel::expression::ValidGrouping<__GB>>::IsAggregate;
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
                #[allow(non_camel_case_types)]
                impl diesel::query_builder::QueryId for id {
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
                #[allow(non_camel_case_types)]
                impl diesel::query_builder::QueryId for title {
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
            #[allow(non_camel_case_types, dead_code)]
            pub struct body;
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for body {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "body")
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for body {
                #[inline]
                fn clone(&self) -> body {
                    *self
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for body {}
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                #[allow(non_camel_case_types)]
                impl diesel::query_builder::QueryId for body {
                    type QueryId = body;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            };
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for body {
                #[inline]
                fn default() -> body {
                    body {}
                }
            }
            impl diesel::expression::Expression for body {
                type SqlType = Text;
            }
            impl<DB> diesel::query_builder::QueryFragment<DB> for body
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
                    __diesel_internal_out.push_identifier("body")
                }
            }
            impl diesel::SelectableExpression<super::table> for body {}
            impl<QS> diesel::AppearsOnTable<QS> for body where
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
                > for body
            where
                body: diesel::AppearsOnTable<
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
                > for body
            where
                body: diesel::AppearsOnTable<
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
                for body
            where
                body: diesel::SelectableExpression<Join>
                    + diesel::AppearsOnTable<diesel::internal::table_macro::JoinOn<Join, On>>,
            {
            }
            impl<From>
                diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for body
            where
                From: diesel::query_source::QuerySource,
                body: diesel::SelectableExpression<From>
                    + diesel::AppearsOnTable<
                        diesel::internal::table_macro::SelectStatement<
                            diesel::internal::table_macro::FromClause<From>,
                        >,
                    >,
            {
            }
            impl<__GB> diesel::expression::ValidGrouping<__GB> for body
            where
                __GB: diesel::expression::IsContainedInGroupBy<
                        body,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
            {
                type IsAggregate = diesel::expression::is_aggregate::Yes;
            }
            impl diesel::expression::ValidGrouping<()> for body {
                type IsAggregate = diesel::expression::is_aggregate::No;
            }
            impl diesel::expression::IsContainedInGroupBy<body> for body {
                type Output = diesel::expression::is_contained_in_group_by::Yes;
            }
            impl diesel::query_source::Column for body {
                type Table = super::table;
                const NAME: &'static str = "body";
            }
            impl<T> diesel::EqAll<T> for body
            where
                T: diesel::expression::AsExpression<Text>,
                diesel::dsl::Eq<body, T::Expression>:
                    diesel::Expression<SqlType = diesel::sql_types::Bool>,
            {
                type Output = diesel::dsl::Eq<Self, T::Expression>;
                fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                    use diesel::expression_methods::ExpressionMethods;
                    self.eq(__diesel_internal_rhs)
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            pub struct published;
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for published {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "published")
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for published {
                #[inline]
                fn clone(&self) -> published {
                    *self
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for published {}
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                #[allow(non_camel_case_types)]
                impl diesel::query_builder::QueryId for published {
                    type QueryId = published;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            };
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for published {
                #[inline]
                fn default() -> published {
                    published {}
                }
            }
            impl diesel::expression::Expression for published {
                type SqlType = Bool;
            }
            impl<DB> diesel::query_builder::QueryFragment<DB> for published
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
                    __diesel_internal_out.push_identifier("published")
                }
            }
            impl diesel::SelectableExpression<super::table> for published {}
            impl<QS> diesel::AppearsOnTable<QS> for published where
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
                > for published
            where
                published: diesel::AppearsOnTable<
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
                > for published
            where
                published: diesel::AppearsOnTable<
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
                for published
            where
                published: diesel::SelectableExpression<Join>
                    + diesel::AppearsOnTable<diesel::internal::table_macro::JoinOn<Join, On>>,
            {
            }
            impl<From>
                diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for published
            where
                From: diesel::query_source::QuerySource,
                published: diesel::SelectableExpression<From>
                    + diesel::AppearsOnTable<
                        diesel::internal::table_macro::SelectStatement<
                            diesel::internal::table_macro::FromClause<From>,
                        >,
                    >,
            {
            }
            impl<__GB> diesel::expression::ValidGrouping<__GB> for published
            where
                __GB: diesel::expression::IsContainedInGroupBy<
                        published,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
            {
                type IsAggregate = diesel::expression::is_aggregate::Yes;
            }
            impl diesel::expression::ValidGrouping<()> for published {
                type IsAggregate = diesel::expression::is_aggregate::No;
            }
            impl diesel::expression::IsContainedInGroupBy<published> for published {
                type Output = diesel::expression::is_contained_in_group_by::Yes;
            }
            impl diesel::query_source::Column for published {
                type Table = super::table;
                const NAME: &'static str = "published";
            }
            impl<T> diesel::EqAll<T> for published
            where
                T: diesel::expression::AsExpression<Bool>,
                diesel::dsl::Eq<published, T::Expression>:
                    diesel::Expression<SqlType = diesel::sql_types::Bool>,
            {
                type Output = diesel::dsl::Eq<Self, T::Expression>;
                fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                    use diesel::expression_methods::ExpressionMethods;
                    self.eq(__diesel_internal_rhs)
                }
            }
            impl diesel::expression::IsContainedInGroupBy<id> for title {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<title> for id {
                type Output = diesel::expression::is_contained_in_group_by::Yes;
            }
            impl diesel::expression::IsContainedInGroupBy<id> for body {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<body> for id {
                type Output = diesel::expression::is_contained_in_group_by::Yes;
            }
            impl diesel::expression::IsContainedInGroupBy<id> for published {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<published> for id {
                type Output = diesel::expression::is_contained_in_group_by::Yes;
            }
            impl diesel::expression::IsContainedInGroupBy<title> for body {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<body> for title {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<title> for published {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<published> for title {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<body> for published {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<published> for body {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
        }
    }
}
pub mod schema {
    #[allow(unused_imports, dead_code, unreachable_pub, unused_qualifications)]
    pub mod posts {
        pub use self::columns::*;
        use ::diesel;
        use diesel::sql_types::*;
        /// Re-exports all of the columns of this table, as well as the
        /// table struct renamed to the module name. This is meant to be
        /// glob imported for functions which only deal with one table.
        pub mod dsl {
            pub use super::columns::body;
            pub use super::columns::id;
            pub use super::columns::published;
            pub use super::columns::title;
            pub use super::table as posts;
        }
        #[allow(non_upper_case_globals, dead_code)]
        /// A tuple of all of the columns on this table
        pub const all_columns: (id, title, body, published) = (id, title, body, published);
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
            #[allow(non_camel_case_types)]
            impl diesel::query_builder::QueryId for table {
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
        pub type SqlType = (Integer, Text, Text, Bool);
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
                &diesel::internal::table_macro::Identifier("posts");
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
            type AllColumns = (id, title, body, published);
            fn primary_key(&self) -> Self::PrimaryKey {
                id
            }
            fn all_columns() -> Self::AllColumns {
                (id, title, body, published)
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
                #[allow(non_camel_case_types)]
                impl diesel::query_builder::QueryId for star {
                    type QueryId = star;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            };
            impl<__GB> diesel::expression::ValidGrouping<__GB> for star
            where
                (id, title, body, published): diesel::expression::ValidGrouping<__GB>,
            {
                type IsAggregate = <(
                    id,
                    title,
                    body,
                    published,
                ) as diesel::expression::ValidGrouping<__GB>>::IsAggregate;
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
                #[allow(non_camel_case_types)]
                impl diesel::query_builder::QueryId for id {
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
                #[allow(non_camel_case_types)]
                impl diesel::query_builder::QueryId for title {
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
            #[allow(non_camel_case_types, dead_code)]
            pub struct body;
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for body {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "body")
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for body {
                #[inline]
                fn clone(&self) -> body {
                    *self
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for body {}
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                #[allow(non_camel_case_types)]
                impl diesel::query_builder::QueryId for body {
                    type QueryId = body;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            };
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for body {
                #[inline]
                fn default() -> body {
                    body {}
                }
            }
            impl diesel::expression::Expression for body {
                type SqlType = Text;
            }
            impl<DB> diesel::query_builder::QueryFragment<DB> for body
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
                    __diesel_internal_out.push_identifier("body")
                }
            }
            impl diesel::SelectableExpression<super::table> for body {}
            impl<QS> diesel::AppearsOnTable<QS> for body where
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
                > for body
            where
                body: diesel::AppearsOnTable<
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
                > for body
            where
                body: diesel::AppearsOnTable<
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
                for body
            where
                body: diesel::SelectableExpression<Join>
                    + diesel::AppearsOnTable<diesel::internal::table_macro::JoinOn<Join, On>>,
            {
            }
            impl<From>
                diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for body
            where
                From: diesel::query_source::QuerySource,
                body: diesel::SelectableExpression<From>
                    + diesel::AppearsOnTable<
                        diesel::internal::table_macro::SelectStatement<
                            diesel::internal::table_macro::FromClause<From>,
                        >,
                    >,
            {
            }
            impl<__GB> diesel::expression::ValidGrouping<__GB> for body
            where
                __GB: diesel::expression::IsContainedInGroupBy<
                        body,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
            {
                type IsAggregate = diesel::expression::is_aggregate::Yes;
            }
            impl diesel::expression::ValidGrouping<()> for body {
                type IsAggregate = diesel::expression::is_aggregate::No;
            }
            impl diesel::expression::IsContainedInGroupBy<body> for body {
                type Output = diesel::expression::is_contained_in_group_by::Yes;
            }
            impl diesel::query_source::Column for body {
                type Table = super::table;
                const NAME: &'static str = "body";
            }
            impl<T> diesel::EqAll<T> for body
            where
                T: diesel::expression::AsExpression<Text>,
                diesel::dsl::Eq<body, T::Expression>:
                    diesel::Expression<SqlType = diesel::sql_types::Bool>,
            {
                type Output = diesel::dsl::Eq<Self, T::Expression>;
                fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                    use diesel::expression_methods::ExpressionMethods;
                    self.eq(__diesel_internal_rhs)
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            pub struct published;
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for published {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "published")
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for published {
                #[inline]
                fn clone(&self) -> published {
                    *self
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for published {}
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                #[allow(non_camel_case_types)]
                impl diesel::query_builder::QueryId for published {
                    type QueryId = published;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            };
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for published {
                #[inline]
                fn default() -> published {
                    published {}
                }
            }
            impl diesel::expression::Expression for published {
                type SqlType = Bool;
            }
            impl<DB> diesel::query_builder::QueryFragment<DB> for published
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
                    __diesel_internal_out.push_identifier("published")
                }
            }
            impl diesel::SelectableExpression<super::table> for published {}
            impl<QS> diesel::AppearsOnTable<QS> for published where
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
                > for published
            where
                published: diesel::AppearsOnTable<
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
                > for published
            where
                published: diesel::AppearsOnTable<
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
                for published
            where
                published: diesel::SelectableExpression<Join>
                    + diesel::AppearsOnTable<diesel::internal::table_macro::JoinOn<Join, On>>,
            {
            }
            impl<From>
                diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for published
            where
                From: diesel::query_source::QuerySource,
                published: diesel::SelectableExpression<From>
                    + diesel::AppearsOnTable<
                        diesel::internal::table_macro::SelectStatement<
                            diesel::internal::table_macro::FromClause<From>,
                        >,
                    >,
            {
            }
            impl<__GB> diesel::expression::ValidGrouping<__GB> for published
            where
                __GB: diesel::expression::IsContainedInGroupBy<
                        published,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
            {
                type IsAggregate = diesel::expression::is_aggregate::Yes;
            }
            impl diesel::expression::ValidGrouping<()> for published {
                type IsAggregate = diesel::expression::is_aggregate::No;
            }
            impl diesel::expression::IsContainedInGroupBy<published> for published {
                type Output = diesel::expression::is_contained_in_group_by::Yes;
            }
            impl diesel::query_source::Column for published {
                type Table = super::table;
                const NAME: &'static str = "published";
            }
            impl<T> diesel::EqAll<T> for published
            where
                T: diesel::expression::AsExpression<Bool>,
                diesel::dsl::Eq<published, T::Expression>:
                    diesel::Expression<SqlType = diesel::sql_types::Bool>,
            {
                type Output = diesel::dsl::Eq<Self, T::Expression>;
                fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                    use diesel::expression_methods::ExpressionMethods;
                    self.eq(__diesel_internal_rhs)
                }
            }
            impl diesel::expression::IsContainedInGroupBy<id> for title {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<title> for id {
                type Output = diesel::expression::is_contained_in_group_by::Yes;
            }
            impl diesel::expression::IsContainedInGroupBy<id> for body {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<body> for id {
                type Output = diesel::expression::is_contained_in_group_by::Yes;
            }
            impl diesel::expression::IsContainedInGroupBy<id> for published {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<published> for id {
                type Output = diesel::expression::is_contained_in_group_by::Yes;
            }
            impl diesel::expression::IsContainedInGroupBy<title> for body {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<body> for title {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<title> for published {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<published> for title {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<body> for published {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<published> for body {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
        }
    }
}
pub mod schema {
    #[allow(unused_imports, dead_code, unreachable_pub, unused_qualifications)]
    pub mod posts {
        pub use self::columns::*;
        use ::diesel;
        use diesel::sql_types::*;
        /// Re-exports all of the columns of this table, as well as the
        /// table struct renamed to the module name. This is meant to be
        /// glob imported for functions which only deal with one table.
        pub mod dsl {
            pub use super::columns::body;
            pub use super::columns::id;
            pub use super::columns::published;
            pub use super::columns::title;
            pub use super::table as posts;
        }
        #[allow(non_upper_case_globals, dead_code)]
        /// A tuple of all of the columns on this table
        pub const all_columns: (id, title, body, published) = (id, title, body, published);
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
            #[allow(non_camel_case_types)]
            impl diesel::query_builder::QueryId for table {
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
        pub type SqlType = (Integer, Text, Text, Bool);
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
                &diesel::internal::table_macro::Identifier("posts");
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
            type AllColumns = (id, title, body, published);
            fn primary_key(&self) -> Self::PrimaryKey {
                id
            }
            fn all_columns() -> Self::AllColumns {
                (id, title, body, published)
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
                #[allow(non_camel_case_types)]
                impl diesel::query_builder::QueryId for star {
                    type QueryId = star;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            };
            impl<__GB> diesel::expression::ValidGrouping<__GB> for star
            where
                (id, title, body, published): diesel::expression::ValidGrouping<__GB>,
            {
                type IsAggregate = <(
                    id,
                    title,
                    body,
                    published,
                ) as diesel::expression::ValidGrouping<__GB>>::IsAggregate;
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
                #[allow(non_camel_case_types)]
                impl diesel::query_builder::QueryId for id {
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
                #[allow(non_camel_case_types)]
                impl diesel::query_builder::QueryId for title {
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
            #[allow(non_camel_case_types, dead_code)]
            pub struct body;
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for body {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "body")
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for body {
                #[inline]
                fn clone(&self) -> body {
                    *self
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for body {}
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                #[allow(non_camel_case_types)]
                impl diesel::query_builder::QueryId for body {
                    type QueryId = body;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            };
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for body {
                #[inline]
                fn default() -> body {
                    body {}
                }
            }
            impl diesel::expression::Expression for body {
                type SqlType = Text;
            }
            impl<DB> diesel::query_builder::QueryFragment<DB> for body
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
                    __diesel_internal_out.push_identifier("body")
                }
            }
            impl diesel::SelectableExpression<super::table> for body {}
            impl<QS> diesel::AppearsOnTable<QS> for body where
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
                > for body
            where
                body: diesel::AppearsOnTable<
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
                > for body
            where
                body: diesel::AppearsOnTable<
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
                for body
            where
                body: diesel::SelectableExpression<Join>
                    + diesel::AppearsOnTable<diesel::internal::table_macro::JoinOn<Join, On>>,
            {
            }
            impl<From>
                diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for body
            where
                From: diesel::query_source::QuerySource,
                body: diesel::SelectableExpression<From>
                    + diesel::AppearsOnTable<
                        diesel::internal::table_macro::SelectStatement<
                            diesel::internal::table_macro::FromClause<From>,
                        >,
                    >,
            {
            }
            impl<__GB> diesel::expression::ValidGrouping<__GB> for body
            where
                __GB: diesel::expression::IsContainedInGroupBy<
                        body,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
            {
                type IsAggregate = diesel::expression::is_aggregate::Yes;
            }
            impl diesel::expression::ValidGrouping<()> for body {
                type IsAggregate = diesel::expression::is_aggregate::No;
            }
            impl diesel::expression::IsContainedInGroupBy<body> for body {
                type Output = diesel::expression::is_contained_in_group_by::Yes;
            }
            impl diesel::query_source::Column for body {
                type Table = super::table;
                const NAME: &'static str = "body";
            }
            impl<T> diesel::EqAll<T> for body
            where
                T: diesel::expression::AsExpression<Text>,
                diesel::dsl::Eq<body, T::Expression>:
                    diesel::Expression<SqlType = diesel::sql_types::Bool>,
            {
                type Output = diesel::dsl::Eq<Self, T::Expression>;
                fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                    use diesel::expression_methods::ExpressionMethods;
                    self.eq(__diesel_internal_rhs)
                }
            }
            #[allow(non_camel_case_types, dead_code)]
            pub struct published;
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for published {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "published")
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for published {
                #[inline]
                fn clone(&self) -> published {
                    *self
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for published {}
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                #[allow(non_camel_case_types)]
                impl diesel::query_builder::QueryId for published {
                    type QueryId = published;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            };
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for published {
                #[inline]
                fn default() -> published {
                    published {}
                }
            }
            impl diesel::expression::Expression for published {
                type SqlType = Bool;
            }
            impl<DB> diesel::query_builder::QueryFragment<DB> for published
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
                    __diesel_internal_out.push_identifier("published")
                }
            }
            impl diesel::SelectableExpression<super::table> for published {}
            impl<QS> diesel::AppearsOnTable<QS> for published where
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
                > for published
            where
                published: diesel::AppearsOnTable<
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
                > for published
            where
                published: diesel::AppearsOnTable<
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
                for published
            where
                published: diesel::SelectableExpression<Join>
                    + diesel::AppearsOnTable<diesel::internal::table_macro::JoinOn<Join, On>>,
            {
            }
            impl<From>
                diesel::SelectableExpression<
                    diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<From>,
                    >,
                > for published
            where
                From: diesel::query_source::QuerySource,
                published: diesel::SelectableExpression<From>
                    + diesel::AppearsOnTable<
                        diesel::internal::table_macro::SelectStatement<
                            diesel::internal::table_macro::FromClause<From>,
                        >,
                    >,
            {
            }
            impl<__GB> diesel::expression::ValidGrouping<__GB> for published
            where
                __GB: diesel::expression::IsContainedInGroupBy<
                        published,
                        Output = diesel::expression::is_contained_in_group_by::Yes,
                    >,
            {
                type IsAggregate = diesel::expression::is_aggregate::Yes;
            }
            impl diesel::expression::ValidGrouping<()> for published {
                type IsAggregate = diesel::expression::is_aggregate::No;
            }
            impl diesel::expression::IsContainedInGroupBy<published> for published {
                type Output = diesel::expression::is_contained_in_group_by::Yes;
            }
            impl diesel::query_source::Column for published {
                type Table = super::table;
                const NAME: &'static str = "published";
            }
            impl<T> diesel::EqAll<T> for published
            where
                T: diesel::expression::AsExpression<Bool>,
                diesel::dsl::Eq<published, T::Expression>:
                    diesel::Expression<SqlType = diesel::sql_types::Bool>,
            {
                type Output = diesel::dsl::Eq<Self, T::Expression>;
                fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                    use diesel::expression_methods::ExpressionMethods;
                    self.eq(__diesel_internal_rhs)
                }
            }
            impl diesel::expression::IsContainedInGroupBy<id> for title {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<title> for id {
                type Output = diesel::expression::is_contained_in_group_by::Yes;
            }
            impl diesel::expression::IsContainedInGroupBy<id> for body {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<body> for id {
                type Output = diesel::expression::is_contained_in_group_by::Yes;
            }
            impl diesel::expression::IsContainedInGroupBy<id> for published {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<published> for id {
                type Output = diesel::expression::is_contained_in_group_by::Yes;
            }
            impl diesel::expression::IsContainedInGroupBy<title> for body {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<body> for title {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<title> for published {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<published> for title {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<body> for published {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<published> for body {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
        }
    }
}
