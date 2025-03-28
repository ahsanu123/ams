pub mod database_metadata {
    use ams_macro::{GenerateDieselTable, GenerateTableEnum};
    use chrono::{NaiveDate, Utc};
    use diesel::{prelude::*, Selectable};
    use sea_query::{Alias, ColumnDef, Iden, SeaRc, Table, TableRef};
    use crate::{
        helper::sql_connection_helper::create_connection,
        migration::migration_trait::Migrationable,
    };
    #[diesel(table_name = metadata_table)]
    pub struct MetadataNoId {
        pub version: i64,
        pub description: String,
    }
    #[allow(unused_imports)]
    const _: () = {
        use diesel;
        use diesel::insertable::Insertable;
        use diesel::internal::derives::insertable::UndecoratedInsertRecord;
        use diesel::prelude::*;
        #[allow(unused_qualifications)]
        impl Insertable<metadata_table::table> for MetadataNoId {
            type Values = <(
                std::option::Option<diesel::dsl::Eq<metadata_table::version, i64>>,
                std::option::Option<diesel::dsl::Eq<metadata_table::description, String>>,
            ) as Insertable<metadata_table::table>>::Values;
            fn values(
                self,
            ) -> <(
                std::option::Option<diesel::dsl::Eq<metadata_table::version, i64>>,
                std::option::Option<diesel::dsl::Eq<metadata_table::description, String>>,
            ) as Insertable<metadata_table::table>>::Values {
                (
                    std::option::Option::Some(metadata_table::version.eq(self.version)),
                    std::option::Option::Some(
                        metadata_table::description.eq(self.description),
                    ),
                )
                    .values()
            }
        }
        #[allow(unused_qualifications)]
        impl<'insert> Insertable<metadata_table::table> for &'insert MetadataNoId {
            type Values = <(
                std::option::Option<
                    diesel::dsl::Eq<metadata_table::version, &'insert i64>,
                >,
                std::option::Option<
                    diesel::dsl::Eq<metadata_table::description, &'insert String>,
                >,
            ) as Insertable<metadata_table::table>>::Values;
            fn values(
                self,
            ) -> <(
                std::option::Option<
                    diesel::dsl::Eq<metadata_table::version, &'insert i64>,
                >,
                std::option::Option<
                    diesel::dsl::Eq<metadata_table::description, &'insert String>,
                >,
            ) as Insertable<metadata_table::table>>::Values {
                (
                    std::option::Option::Some(metadata_table::version.eq(&self.version)),
                    std::option::Option::Some(
                        metadata_table::description.eq(&self.description),
                    ),
                )
                    .values()
            }
        }
        impl UndecoratedInsertRecord<metadata_table::table> for MetadataNoId {}
    };
    #[diesel(table_name = metadata_table)]
    pub struct Metadata {
        pub id: i32,
        pub version: i64,
        pub description: String,
    }
    pub enum MetadataTable {
        Id,
        Version,
        Description,
        Table,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for MetadataTable {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    MetadataTable::Id => "Id",
                    MetadataTable::Version => "Version",
                    MetadataTable::Description => "Description",
                    MetadataTable::Table => "Table",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for MetadataTable {
        #[inline]
        fn clone(&self) -> MetadataTable {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for MetadataTable {}
    impl sea_query::Iden for MetadataTable {
        fn prepare(&self, s: &mut dyn ::std::fmt::Write, q: sea_query::Quote) {
            s.write_fmt(format_args!("{0}", q.left())).unwrap();
            self.unquoted(s);
            s.write_fmt(format_args!("{0}", q.right())).unwrap();
        }
        fn unquoted(&self, s: &mut dyn ::std::fmt::Write) {
            match self {
                Self::Id => s.write_fmt(format_args!("{0}", "id")).unwrap(),
                Self::Version => s.write_fmt(format_args!("{0}", "version")).unwrap(),
                Self::Description => {
                    s.write_fmt(format_args!("{0}", "description")).unwrap()
                }
                Self::Table => {
                    s.write_fmt(format_args!("{0}", "metadata_table")).unwrap()
                }
            };
        }
    }
    #[allow(unused_imports, dead_code, unreachable_pub, unused_qualifications)]
    pub mod metadata_table {
        use ::diesel;
        pub use self::columns::*;
        use diesel::sql_types::*;
        /// Re-exports all of the columns of this table, as well as the
        /// table struct renamed to the module name. This is meant to be
        /// glob imported for functions which only deal with one table.
        pub mod dsl {
            pub use super::columns::id;
            pub use super::columns::version;
            pub use super::columns::description;
            pub use super::table as metadata_table;
        }
        #[allow(non_upper_case_globals, dead_code)]
        /// A tuple of all of the columns on this table
        pub const all_columns: (id, version, description) = (id, version, description);
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
        pub type SqlType = (Integer, BigInt, Text);
        /// Helper type for representing a boxed query from this table
        pub type BoxedQuery<'a, DB, ST = SqlType> = diesel::internal::table_macro::BoxedSelectStatement<
            'a,
            ST,
            diesel::internal::table_macro::FromClause<table>,
            DB,
        >;
        impl diesel::QuerySource for table {
            type FromClause = diesel::internal::table_macro::StaticQueryFragmentInstance<
                table,
            >;
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
            <table as diesel::internal::table_macro::StaticQueryFragment>::Component: diesel::query_builder::QueryFragment<
                DB,
            >,
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
            const STATIC_COMPONENT: &'static Self::Component = &diesel::internal::table_macro::Identifier(
                "metadata_table",
            );
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
            type AllColumns = (id, version, description);
            fn primary_key(&self) -> Self::PrimaryKey {
                id
            }
            fn all_columns() -> Self::AllColumns {
                (id, version, description)
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
        impl<S> diesel::internal::table_macro::AliasAppearsInFromClause<S, table>
        for table
        where
            S: diesel::query_source::AliasSource<Target = table>,
        {
            type Count = diesel::query_source::Never;
        }
        impl<
            S1,
            S2,
        > diesel::internal::table_macro::AliasAliasAppearsInFromClause<table, S2, S1>
        for table
        where
            S1: diesel::query_source::AliasSource<Target = table>,
            S2: diesel::query_source::AliasSource<Target = table>,
            S1: diesel::internal::table_macro::AliasAliasAppearsInFromClauseSameTable<
                S2,
                table,
            >,
        {
            type Count = <S1 as diesel::internal::table_macro::AliasAliasAppearsInFromClauseSameTable<
                S2,
                table,
            >>::Count;
        }
        impl<S> diesel::query_source::AppearsInFromClause<diesel::query_source::Alias<S>>
        for table
        where
            S: diesel::query_source::AliasSource,
        {
            type Count = diesel::query_source::Never;
        }
        impl<
            S,
            C,
        > diesel::internal::table_macro::FieldAliasMapperAssociatedTypesDisjointnessTrick<
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
        for diesel::internal::table_macro::NoFromClause {
            type Count = diesel::query_source::Never;
        }
        impl<
            Left,
            Right,
            Kind,
        > diesel::JoinTo<diesel::internal::table_macro::Join<Left, Right, Kind>>
        for table
        where
            diesel::internal::table_macro::Join<
                Left,
                Right,
                Kind,
            >: diesel::JoinTo<table>,
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
                __diesel_internal_rhs: diesel::internal::table_macro::Join<
                    Left,
                    Right,
                    Kind,
                >,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, __diesel_internal_on_clause) = diesel::internal::table_macro::Join::join_target(
                    table,
                );
                (__diesel_internal_rhs, __diesel_internal_on_clause)
            }
        }
        impl<Join, On> diesel::JoinTo<diesel::internal::table_macro::JoinOn<Join, On>>
        for table
        where
            diesel::internal::table_macro::JoinOn<Join, On>: diesel::JoinTo<table>,
        {
            type FromClause = diesel::internal::table_macro::JoinOn<Join, On>;
            type OnClause = <diesel::internal::table_macro::JoinOn<
                Join,
                On,
            > as diesel::JoinTo<table>>::OnClause;
            fn join_target(
                __diesel_internal_rhs: diesel::internal::table_macro::JoinOn<Join, On>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, __diesel_internal_on_clause) = diesel::internal::table_macro::JoinOn::join_target(
                    table,
                );
                (__diesel_internal_rhs, __diesel_internal_on_clause)
            }
        }
        impl<
            F,
            S,
            D,
            W,
            O,
            L,
            Of,
            G,
        > diesel::JoinTo<
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
                let (_, __diesel_internal_on_clause) = diesel::internal::table_macro::SelectStatement::join_target(
                    table,
                );
                (__diesel_internal_rhs, __diesel_internal_on_clause)
            }
        }
        impl<
            'a,
            QS,
            ST,
            DB,
        > diesel::JoinTo<
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
                let (_, __diesel_internal_on_clause) = diesel::internal::table_macro::BoxedSelectStatement::join_target(
                    table,
                );
                (__diesel_internal_rhs, __diesel_internal_on_clause)
            }
        }
        impl<S> diesel::JoinTo<diesel::query_source::Alias<S>> for table
        where
            diesel::query_source::Alias<S>: diesel::JoinTo<table>,
        {
            type FromClause = diesel::query_source::Alias<S>;
            type OnClause = <diesel::query_source::Alias<
                S,
            > as diesel::JoinTo<table>>::OnClause;
            fn join_target(
                __diesel_internal_rhs: diesel::query_source::Alias<S>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, __diesel_internal_on_clause) = diesel::query_source::Alias::<
                    S,
                >::join_target(table);
                (__diesel_internal_rhs, __diesel_internal_on_clause)
            }
        }
        impl<T> diesel::insertable::Insertable<T> for table
        where
            <table as diesel::query_builder::AsQuery>::Query: diesel::insertable::Insertable<
                T,
            >,
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
            type OnClause = <diesel::query_builder::Only<
                S,
            > as diesel::JoinTo<table>>::OnClause;
            fn join_target(
                __diesel_internal_rhs: diesel::query_builder::Only<S>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, __diesel_internal_on_clause) = diesel::query_builder::Only::<
                    S,
                >::join_target(table);
                (__diesel_internal_rhs, __diesel_internal_on_clause)
            }
        }
        impl diesel::query_source::AppearsInFromClause<
            diesel::query_builder::Only<table>,
        > for table {
            type Count = diesel::query_source::Once;
        }
        impl diesel::query_source::AppearsInFromClause<table>
        for diesel::query_builder::Only<table> {
            type Count = diesel::query_source::Once;
        }
        impl<S, TSM> diesel::JoinTo<diesel::query_builder::Tablesample<S, TSM>> for table
        where
            diesel::query_builder::Tablesample<S, TSM>: diesel::JoinTo<table>,
            TSM: diesel::internal::table_macro::TablesampleMethod,
        {
            type FromClause = diesel::query_builder::Tablesample<S, TSM>;
            type OnClause = <diesel::query_builder::Tablesample<
                S,
                TSM,
            > as diesel::JoinTo<table>>::OnClause;
            fn join_target(
                __diesel_internal_rhs: diesel::query_builder::Tablesample<S, TSM>,
            ) -> (Self::FromClause, Self::OnClause) {
                let (_, __diesel_internal_on_clause) = diesel::query_builder::Tablesample::<
                    S,
                    TSM,
                >::join_target(table);
                (__diesel_internal_rhs, __diesel_internal_on_clause)
            }
        }
        impl<
            TSM,
        > diesel::query_source::AppearsInFromClause<
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
            use ::diesel;
            use super::table;
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
                (id, version, description): diesel::expression::ValidGrouping<__GB>,
            {
                type IsAggregate = <(
                    id,
                    version,
                    description,
                ) as diesel::expression::ValidGrouping<__GB>>::IsAggregate;
            }
            impl diesel::Expression for star {
                type SqlType = diesel::expression::expression_types::NotSelectable;
            }
            impl<DB: diesel::backend::Backend> diesel::query_builder::QueryFragment<DB>
            for star
            where
                <table as diesel::QuerySource>::FromClause: diesel::query_builder::QueryFragment<
                    DB,
                >,
            {
                #[allow(non_snake_case)]
                fn walk_ast<'b>(
                    &'b self,
                    mut __diesel_internal_out: diesel::query_builder::AstPass<'_, 'b, DB>,
                ) -> diesel::result::QueryResult<()> {
                    use diesel::QuerySource;
                    if !__diesel_internal_out.should_skip_from() {
                        const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                            table,
                        > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
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
                diesel::internal::table_macro::StaticQueryFragmentInstance<
                    table,
                >: diesel::query_builder::QueryFragment<DB>,
            {
                #[allow(non_snake_case)]
                fn walk_ast<'b>(
                    &'b self,
                    mut __diesel_internal_out: diesel::query_builder::AstPass<'_, 'b, DB>,
                ) -> diesel::result::QueryResult<()> {
                    if !__diesel_internal_out.should_skip_from() {
                        const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                            table,
                        > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                        FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                        __diesel_internal_out.push_sql(".");
                    }
                    __diesel_internal_out.push_identifier("id")
                }
            }
            impl diesel::SelectableExpression<super::table> for id {}
            impl<QS> diesel::AppearsOnTable<QS> for id
            where
                QS: diesel::query_source::AppearsInFromClause<
                    super::table,
                    Count = diesel::query_source::Once,
                >,
            {}
            impl<
                Left,
                Right,
            > diesel::SelectableExpression<
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
            {}
            impl<
                Left,
                Right,
            > diesel::SelectableExpression<
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
                (
                    Left::Count,
                    Right::Count,
                ): diesel::internal::table_macro::Pick<Left, Right>,
                Self: diesel::SelectableExpression<
                    <(
                        Left::Count,
                        Right::Count,
                    ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
                >,
            {}
            impl<
                Join,
                On,
            > diesel::SelectableExpression<
                diesel::internal::table_macro::JoinOn<Join, On>,
            > for id
            where
                id: diesel::SelectableExpression<Join>
                    + diesel::AppearsOnTable<
                        diesel::internal::table_macro::JoinOn<Join, On>,
                    >,
            {}
            impl<
                From,
            > diesel::SelectableExpression<
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
            {}
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
                diesel::dsl::Eq<
                    id,
                    T::Expression,
                >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
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
            impl<Rhs> ::std::ops::Sub<Rhs> for id
            where
                Rhs: diesel::expression::AsExpression<
                    <<id as diesel::Expression>::SqlType as diesel::sql_types::ops::Sub>::Rhs,
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
            impl<Rhs> ::std::ops::Div<Rhs> for id
            where
                Rhs: diesel::expression::AsExpression<
                    <<id as diesel::Expression>::SqlType as diesel::sql_types::ops::Div>::Rhs,
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
            impl<Rhs> ::std::ops::Mul<Rhs> for id
            where
                Rhs: diesel::expression::AsExpression<
                    <<id as diesel::Expression>::SqlType as diesel::sql_types::ops::Mul>::Rhs,
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
            impl diesel::query_source::AppearsInFromClause<
                diesel::query_builder::Only<super::table>,
            > for id {
                type Count = diesel::query_source::Once;
            }
            impl diesel::SelectableExpression<diesel::query_builder::Only<super::table>>
            for id {}
            impl<
                TSM,
            > diesel::query_source::AppearsInFromClause<
                diesel::query_builder::Tablesample<super::table, TSM>,
            > for id
            where
                TSM: diesel::internal::table_macro::TablesampleMethod,
            {
                type Count = diesel::query_source::Once;
            }
            impl<
                TSM,
            > diesel::SelectableExpression<
                diesel::query_builder::Tablesample<super::table, TSM>,
            > for id
            where
                TSM: diesel::internal::table_macro::TablesampleMethod,
            {}
            #[allow(non_camel_case_types, dead_code)]
            pub struct version;
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for version {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "version")
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for version {
                #[inline]
                fn clone(&self) -> version {
                    *self
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for version {}
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for version {
                    type QueryId = version;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            };
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for version {
                #[inline]
                fn default() -> version {
                    version {}
                }
            }
            impl diesel::expression::Expression for version {
                type SqlType = BigInt;
            }
            impl<DB> diesel::query_builder::QueryFragment<DB> for version
            where
                DB: diesel::backend::Backend,
                diesel::internal::table_macro::StaticQueryFragmentInstance<
                    table,
                >: diesel::query_builder::QueryFragment<DB>,
            {
                #[allow(non_snake_case)]
                fn walk_ast<'b>(
                    &'b self,
                    mut __diesel_internal_out: diesel::query_builder::AstPass<'_, 'b, DB>,
                ) -> diesel::result::QueryResult<()> {
                    if !__diesel_internal_out.should_skip_from() {
                        const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                            table,
                        > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                        FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                        __diesel_internal_out.push_sql(".");
                    }
                    __diesel_internal_out.push_identifier("version")
                }
            }
            impl diesel::SelectableExpression<super::table> for version {}
            impl<QS> diesel::AppearsOnTable<QS> for version
            where
                QS: diesel::query_source::AppearsInFromClause<
                    super::table,
                    Count = diesel::query_source::Once,
                >,
            {}
            impl<
                Left,
                Right,
            > diesel::SelectableExpression<
                diesel::internal::table_macro::Join<
                    Left,
                    Right,
                    diesel::internal::table_macro::LeftOuter,
                >,
            > for version
            where
                version: diesel::AppearsOnTable<
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
            {}
            impl<
                Left,
                Right,
            > diesel::SelectableExpression<
                diesel::internal::table_macro::Join<
                    Left,
                    Right,
                    diesel::internal::table_macro::Inner,
                >,
            > for version
            where
                version: diesel::AppearsOnTable<
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
                (
                    Left::Count,
                    Right::Count,
                ): diesel::internal::table_macro::Pick<Left, Right>,
                Self: diesel::SelectableExpression<
                    <(
                        Left::Count,
                        Right::Count,
                    ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
                >,
            {}
            impl<
                Join,
                On,
            > diesel::SelectableExpression<
                diesel::internal::table_macro::JoinOn<Join, On>,
            > for version
            where
                version: diesel::SelectableExpression<Join>
                    + diesel::AppearsOnTable<
                        diesel::internal::table_macro::JoinOn<Join, On>,
                    >,
            {}
            impl<
                From,
            > diesel::SelectableExpression<
                diesel::internal::table_macro::SelectStatement<
                    diesel::internal::table_macro::FromClause<From>,
                >,
            > for version
            where
                From: diesel::query_source::QuerySource,
                version: diesel::SelectableExpression<From>
                    + diesel::AppearsOnTable<
                        diesel::internal::table_macro::SelectStatement<
                            diesel::internal::table_macro::FromClause<From>,
                        >,
                    >,
            {}
            impl<__GB> diesel::expression::ValidGrouping<__GB> for version
            where
                __GB: diesel::expression::IsContainedInGroupBy<
                    version,
                    Output = diesel::expression::is_contained_in_group_by::Yes,
                >,
            {
                type IsAggregate = diesel::expression::is_aggregate::Yes;
            }
            impl diesel::expression::ValidGrouping<()> for version {
                type IsAggregate = diesel::expression::is_aggregate::No;
            }
            impl diesel::expression::IsContainedInGroupBy<version> for version {
                type Output = diesel::expression::is_contained_in_group_by::Yes;
            }
            impl diesel::query_source::Column for version {
                type Table = super::table;
                const NAME: &'static str = "version";
            }
            impl<T> diesel::EqAll<T> for version
            where
                T: diesel::expression::AsExpression<BigInt>,
                diesel::dsl::Eq<
                    version,
                    T::Expression,
                >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
            {
                type Output = diesel::dsl::Eq<Self, T::Expression>;
                fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                    use diesel::expression_methods::ExpressionMethods;
                    self.eq(__diesel_internal_rhs)
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for version
            where
                Rhs: diesel::expression::AsExpression<
                    <<version as diesel::Expression>::SqlType as diesel::sql_types::ops::Add>::Rhs,
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
            impl<Rhs> ::std::ops::Sub<Rhs> for version
            where
                Rhs: diesel::expression::AsExpression<
                    <<version as diesel::Expression>::SqlType as diesel::sql_types::ops::Sub>::Rhs,
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
            impl<Rhs> ::std::ops::Div<Rhs> for version
            where
                Rhs: diesel::expression::AsExpression<
                    <<version as diesel::Expression>::SqlType as diesel::sql_types::ops::Div>::Rhs,
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
            impl<Rhs> ::std::ops::Mul<Rhs> for version
            where
                Rhs: diesel::expression::AsExpression<
                    <<version as diesel::Expression>::SqlType as diesel::sql_types::ops::Mul>::Rhs,
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
            impl diesel::query_source::AppearsInFromClause<
                diesel::query_builder::Only<super::table>,
            > for version {
                type Count = diesel::query_source::Once;
            }
            impl diesel::SelectableExpression<diesel::query_builder::Only<super::table>>
            for version {}
            impl<
                TSM,
            > diesel::query_source::AppearsInFromClause<
                diesel::query_builder::Tablesample<super::table, TSM>,
            > for version
            where
                TSM: diesel::internal::table_macro::TablesampleMethod,
            {
                type Count = diesel::query_source::Once;
            }
            impl<
                TSM,
            > diesel::SelectableExpression<
                diesel::query_builder::Tablesample<super::table, TSM>,
            > for version
            where
                TSM: diesel::internal::table_macro::TablesampleMethod,
            {}
            #[allow(non_camel_case_types, dead_code)]
            pub struct description;
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for description {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "description")
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for description {
                #[inline]
                fn clone(&self) -> description {
                    *self
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for description {}
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for description {
                    type QueryId = description;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            };
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for description {
                #[inline]
                fn default() -> description {
                    description {}
                }
            }
            impl diesel::expression::Expression for description {
                type SqlType = Text;
            }
            impl<DB> diesel::query_builder::QueryFragment<DB> for description
            where
                DB: diesel::backend::Backend,
                diesel::internal::table_macro::StaticQueryFragmentInstance<
                    table,
                >: diesel::query_builder::QueryFragment<DB>,
            {
                #[allow(non_snake_case)]
                fn walk_ast<'b>(
                    &'b self,
                    mut __diesel_internal_out: diesel::query_builder::AstPass<'_, 'b, DB>,
                ) -> diesel::result::QueryResult<()> {
                    if !__diesel_internal_out.should_skip_from() {
                        const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<
                            table,
                        > = diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                        FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                        __diesel_internal_out.push_sql(".");
                    }
                    __diesel_internal_out.push_identifier("description")
                }
            }
            impl diesel::SelectableExpression<super::table> for description {}
            impl<QS> diesel::AppearsOnTable<QS> for description
            where
                QS: diesel::query_source::AppearsInFromClause<
                    super::table,
                    Count = diesel::query_source::Once,
                >,
            {}
            impl<
                Left,
                Right,
            > diesel::SelectableExpression<
                diesel::internal::table_macro::Join<
                    Left,
                    Right,
                    diesel::internal::table_macro::LeftOuter,
                >,
            > for description
            where
                description: diesel::AppearsOnTable<
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
            {}
            impl<
                Left,
                Right,
            > diesel::SelectableExpression<
                diesel::internal::table_macro::Join<
                    Left,
                    Right,
                    diesel::internal::table_macro::Inner,
                >,
            > for description
            where
                description: diesel::AppearsOnTable<
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
                (
                    Left::Count,
                    Right::Count,
                ): diesel::internal::table_macro::Pick<Left, Right>,
                Self: diesel::SelectableExpression<
                    <(
                        Left::Count,
                        Right::Count,
                    ) as diesel::internal::table_macro::Pick<Left, Right>>::Selection,
                >,
            {}
            impl<
                Join,
                On,
            > diesel::SelectableExpression<
                diesel::internal::table_macro::JoinOn<Join, On>,
            > for description
            where
                description: diesel::SelectableExpression<Join>
                    + diesel::AppearsOnTable<
                        diesel::internal::table_macro::JoinOn<Join, On>,
                    >,
            {}
            impl<
                From,
            > diesel::SelectableExpression<
                diesel::internal::table_macro::SelectStatement<
                    diesel::internal::table_macro::FromClause<From>,
                >,
            > for description
            where
                From: diesel::query_source::QuerySource,
                description: diesel::SelectableExpression<From>
                    + diesel::AppearsOnTable<
                        diesel::internal::table_macro::SelectStatement<
                            diesel::internal::table_macro::FromClause<From>,
                        >,
                    >,
            {}
            impl<__GB> diesel::expression::ValidGrouping<__GB> for description
            where
                __GB: diesel::expression::IsContainedInGroupBy<
                    description,
                    Output = diesel::expression::is_contained_in_group_by::Yes,
                >,
            {
                type IsAggregate = diesel::expression::is_aggregate::Yes;
            }
            impl diesel::expression::ValidGrouping<()> for description {
                type IsAggregate = diesel::expression::is_aggregate::No;
            }
            impl diesel::expression::IsContainedInGroupBy<description> for description {
                type Output = diesel::expression::is_contained_in_group_by::Yes;
            }
            impl diesel::query_source::Column for description {
                type Table = super::table;
                const NAME: &'static str = "description";
            }
            impl<T> diesel::EqAll<T> for description
            where
                T: diesel::expression::AsExpression<Text>,
                diesel::dsl::Eq<
                    description,
                    T::Expression,
                >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
            {
                type Output = diesel::dsl::Eq<Self, T::Expression>;
                fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                    use diesel::expression_methods::ExpressionMethods;
                    self.eq(__diesel_internal_rhs)
                }
            }
            impl diesel::query_source::AppearsInFromClause<
                diesel::query_builder::Only<super::table>,
            > for description {
                type Count = diesel::query_source::Once;
            }
            impl diesel::SelectableExpression<diesel::query_builder::Only<super::table>>
            for description {}
            impl<
                TSM,
            > diesel::query_source::AppearsInFromClause<
                diesel::query_builder::Tablesample<super::table, TSM>,
            > for description
            where
                TSM: diesel::internal::table_macro::TablesampleMethod,
            {
                type Count = diesel::query_source::Once;
            }
            impl<
                TSM,
            > diesel::SelectableExpression<
                diesel::query_builder::Tablesample<super::table, TSM>,
            > for description
            where
                TSM: diesel::internal::table_macro::TablesampleMethod,
            {}
            impl diesel::expression::IsContainedInGroupBy<id> for version {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<version> for id {
                type Output = diesel::expression::is_contained_in_group_by::Yes;
            }
            impl diesel::expression::IsContainedInGroupBy<id> for description {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<description> for id {
                type Output = diesel::expression::is_contained_in_group_by::Yes;
            }
            impl diesel::expression::IsContainedInGroupBy<version> for description {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<description> for version {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
        }
    }
    #[allow(unused_imports)]
    const _: () = {
        use diesel;
        use diesel::deserialize::{self, FromStaticSqlRow, Queryable};
        use diesel::row::{Row as _, Field as _};
        use std::convert::TryInto;
        impl<
            __DB: diesel::backend::Backend,
            __ST0,
            __ST1,
            __ST2,
        > Queryable<(__ST0, __ST1, __ST2), __DB> for Metadata
        where
            (i32, i64, String): FromStaticSqlRow<(__ST0, __ST1, __ST2), __DB>,
        {
            type Row = (i32, i64, String);
            fn build(row: Self::Row) -> deserialize::Result<Self> {
                Ok(Self {
                    id: row.0.try_into()?,
                    version: row.1.try_into()?,
                    description: row.2.try_into()?,
                })
            }
        }
    };
    #[allow(unused_imports)]
    const _: () = {
        use diesel;
        use diesel::associations::{HasTable, Identifiable};
        impl HasTable for Metadata {
            type Table = metadata_table::table;
            fn table() -> Self::Table {
                metadata_table::table
            }
        }
        impl<'ident> Identifiable for &'ident Metadata {
            type Id = (&'ident i32);
            fn id(self) -> Self::Id {
                (&self.id)
            }
        }
        impl<'ident> Identifiable for &'_ &'ident Metadata {
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
        impl<__DB: diesel::backend::Backend> Selectable<__DB> for Metadata {
            type SelectExpression = (
                metadata_table::id,
                metadata_table::version,
                metadata_table::description,
            );
            fn construct_selection() -> Self::SelectExpression {
                (
                    metadata_table::id,
                    metadata_table::version,
                    metadata_table::description,
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for Metadata {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Metadata",
                "id",
                &self.id,
                "version",
                &self.version,
                "description",
                &&self.description,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Metadata {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Metadata {
        #[inline]
        fn eq(&self, other: &Metadata) -> bool {
            self.id == other.id && self.version == other.version
                && self.description == other.description
        }
    }
    impl Migrationable for Metadata {
        fn get_up_migration(builder: impl sea_query::SchemaBuilder) -> String {
            Table::create()
                .table(MetadataTable::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(MetadataTable::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(MetadataTable::Version).integer())
                .col(ColumnDef::new(MetadataTable::Description).text())
                .build(builder)
        }
        fn get_down_migration(builder: impl sea_query::SchemaBuilder) -> String {
            Table::drop().table(MetadataTable::Table).if_exists().build(builder)
        }
    }
    impl Metadata {
        pub fn get_latest_version() -> i32 {
            let conn = &mut create_connection();
            let latest_db_metadata = metadata_table::table
                .order(metadata_table::version.desc())
                .first::<Metadata>(conn);
            match latest_db_metadata {
                Ok(db_metatdata) => db_metatdata.version as i32,
                Err(_) => 0,
            }
        }
        pub fn add_migration_stamp() {
            let conn = &mut create_connection();
            let latest_version = Self::get_latest_version();
            let new_db_metadata = MetadataNoId {
                version: (latest_version + 1) as i64,
                description: Utc::now().naive_utc().to_string(),
            };
            let _ = diesel::insert_into(metadata_table::table)
                .values(new_db_metadata)
                .execute(conn)
                .expect("Cant Insert to database metadata!!!");
        }
    }
}
