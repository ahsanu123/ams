diesel :: table!
{
    product_table(id)
    {
        id -> Integer, user_id -> Integer, paid -> Bool, production_date ->
        Date, taken_date -> Date, price -> Double, amount -> Integer,
        description -> Text,
    }
}
pub mod product {
    use crate::migration::migration_trait::Migrationable;
    use chrono::NaiveDate;
    use custom_macro::{GenerateDieselTable, GenerateTableEnum};
    use diesel::prelude::*;
    use diesel::{prelude::Queryable, Selectable};
    use sea_query::{
        ColumnDef, Iden, IntoValueTuple, SchemaBuilder, SqliteQueryBuilder, Table,
        TableCreateStatement, Value,
    };
    #[diesel(table_name = product_table)]
    pub struct Product {
        pub id: i32,
        pub user_id: i32,
        pub paid: bool,
        pub production_date: NaiveDate,
        pub taken_date: NaiveDate,
        pub price: f64,
        pub amount: i32,
        pub description: String,
    }
    pub enum ProductTable {
        Id,
        UserId,
        Paid,
        ProductionDate,
        TakenDate,
        Price,
        Amount,
        Description,
        Table,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ProductTable {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    ProductTable::Id => "Id",
                    ProductTable::UserId => "UserId",
                    ProductTable::Paid => "Paid",
                    ProductTable::ProductionDate => "ProductionDate",
                    ProductTable::TakenDate => "TakenDate",
                    ProductTable::Price => "Price",
                    ProductTable::Amount => "Amount",
                    ProductTable::Description => "Description",
                    ProductTable::Table => "Table",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ProductTable {
        #[inline]
        fn clone(&self) -> ProductTable {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ProductTable {}
    impl sea_query::Iden for ProductTable {
        fn prepare(&self, s: &mut dyn ::std::fmt::Write, q: sea_query::Quote) {
            s.write_fmt(format_args!("{0}", q.left())).unwrap();
            self.unquoted(s);
            s.write_fmt(format_args!("{0}", q.right())).unwrap();
        }
        fn unquoted(&self, s: &mut dyn ::std::fmt::Write) {
            match self {
                Self::Id => s.write_fmt(format_args!("{0}", "id")).unwrap(),
                Self::UserId => s.write_fmt(format_args!("{0}", "user_id")).unwrap(),
                Self::Paid => s.write_fmt(format_args!("{0}", "paid")).unwrap(),
                Self::ProductionDate => {
                    s.write_fmt(format_args!("{0}", "production_date")).unwrap()
                }
                Self::TakenDate => {
                    s.write_fmt(format_args!("{0}", "taken_date")).unwrap()
                }
                Self::Price => s.write_fmt(format_args!("{0}", "price")).unwrap(),
                Self::Amount => s.write_fmt(format_args!("{0}", "amount")).unwrap(),
                Self::Description => {
                    s.write_fmt(format_args!("{0}", "description")).unwrap()
                }
                Self::Table => s.write_fmt(format_args!("{0}", "product_table")).unwrap(),
            };
        }
    }
    #[allow(unused_imports, dead_code, unreachable_pub, unused_qualifications)]
    pub mod product_table {
        use ::diesel;
        pub use self::columns::*;
        use diesel::sql_types::*;
        /// Re-exports all of the columns of this table, as well as the
        /// table struct renamed to the module name. This is meant to be
        /// glob imported for functions which only deal with one table.
        pub mod dsl {
            pub use super::columns::id;
            pub use super::columns::user_id;
            pub use super::columns::paid;
            pub use super::columns::production_date;
            pub use super::columns::taken_date;
            pub use super::columns::price;
            pub use super::columns::amount;
            pub use super::columns::description;
            pub use super::table as product_table;
        }
        #[allow(non_upper_case_globals, dead_code)]
        /// A tuple of all of the columns on this table
        pub const all_columns: (
            id,
            user_id,
            paid,
            production_date,
            taken_date,
            price,
            amount,
            description,
        ) = (id, user_id, paid, production_date, taken_date, price, amount, description);
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
        pub type SqlType = (Integer, Integer, Bool, Date, Date, Double, Integer, Text);
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
                "product_table",
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
            type AllColumns = (
                id,
                user_id,
                paid,
                production_date,
                taken_date,
                price,
                amount,
                description,
            );
            fn primary_key(&self) -> Self::PrimaryKey {
                id
            }
            fn all_columns() -> Self::AllColumns {
                (
                    id,
                    user_id,
                    paid,
                    production_date,
                    taken_date,
                    price,
                    amount,
                    description,
                )
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
                (
                    id,
                    user_id,
                    paid,
                    production_date,
                    taken_date,
                    price,
                    amount,
                    description,
                ): diesel::expression::ValidGrouping<__GB>,
            {
                type IsAggregate = <(
                    id,
                    user_id,
                    paid,
                    production_date,
                    taken_date,
                    price,
                    amount,
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
            pub struct user_id;
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for user_id {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "user_id")
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for user_id {
                #[inline]
                fn clone(&self) -> user_id {
                    *self
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for user_id {}
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for user_id {
                    type QueryId = user_id;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            };
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for user_id {
                #[inline]
                fn default() -> user_id {
                    user_id {}
                }
            }
            impl diesel::expression::Expression for user_id {
                type SqlType = Integer;
            }
            impl<DB> diesel::query_builder::QueryFragment<DB> for user_id
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
                    __diesel_internal_out.push_identifier("user_id")
                }
            }
            impl diesel::SelectableExpression<super::table> for user_id {}
            impl<QS> diesel::AppearsOnTable<QS> for user_id
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
            > for user_id
            where
                user_id: diesel::AppearsOnTable<
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
            > for user_id
            where
                user_id: diesel::AppearsOnTable<
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
            > for user_id
            where
                user_id: diesel::SelectableExpression<Join>
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
            > for user_id
            where
                From: diesel::query_source::QuerySource,
                user_id: diesel::SelectableExpression<From>
                    + diesel::AppearsOnTable<
                        diesel::internal::table_macro::SelectStatement<
                            diesel::internal::table_macro::FromClause<From>,
                        >,
                    >,
            {}
            impl<__GB> diesel::expression::ValidGrouping<__GB> for user_id
            where
                __GB: diesel::expression::IsContainedInGroupBy<
                    user_id,
                    Output = diesel::expression::is_contained_in_group_by::Yes,
                >,
            {
                type IsAggregate = diesel::expression::is_aggregate::Yes;
            }
            impl diesel::expression::ValidGrouping<()> for user_id {
                type IsAggregate = diesel::expression::is_aggregate::No;
            }
            impl diesel::expression::IsContainedInGroupBy<user_id> for user_id {
                type Output = diesel::expression::is_contained_in_group_by::Yes;
            }
            impl diesel::query_source::Column for user_id {
                type Table = super::table;
                const NAME: &'static str = "user_id";
            }
            impl<T> diesel::EqAll<T> for user_id
            where
                T: diesel::expression::AsExpression<Integer>,
                diesel::dsl::Eq<
                    user_id,
                    T::Expression,
                >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
            {
                type Output = diesel::dsl::Eq<Self, T::Expression>;
                fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                    use diesel::expression_methods::ExpressionMethods;
                    self.eq(__diesel_internal_rhs)
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for user_id
            where
                Rhs: diesel::expression::AsExpression<
                    <<user_id as diesel::Expression>::SqlType as diesel::sql_types::ops::Add>::Rhs,
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
            impl<Rhs> ::std::ops::Sub<Rhs> for user_id
            where
                Rhs: diesel::expression::AsExpression<
                    <<user_id as diesel::Expression>::SqlType as diesel::sql_types::ops::Sub>::Rhs,
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
            impl<Rhs> ::std::ops::Div<Rhs> for user_id
            where
                Rhs: diesel::expression::AsExpression<
                    <<user_id as diesel::Expression>::SqlType as diesel::sql_types::ops::Div>::Rhs,
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
            impl<Rhs> ::std::ops::Mul<Rhs> for user_id
            where
                Rhs: diesel::expression::AsExpression<
                    <<user_id as diesel::Expression>::SqlType as diesel::sql_types::ops::Mul>::Rhs,
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
            > for user_id {
                type Count = diesel::query_source::Once;
            }
            impl diesel::SelectableExpression<diesel::query_builder::Only<super::table>>
            for user_id {}
            impl<
                TSM,
            > diesel::query_source::AppearsInFromClause<
                diesel::query_builder::Tablesample<super::table, TSM>,
            > for user_id
            where
                TSM: diesel::internal::table_macro::TablesampleMethod,
            {
                type Count = diesel::query_source::Once;
            }
            impl<
                TSM,
            > diesel::SelectableExpression<
                diesel::query_builder::Tablesample<super::table, TSM>,
            > for user_id
            where
                TSM: diesel::internal::table_macro::TablesampleMethod,
            {}
            #[allow(non_camel_case_types, dead_code)]
            pub struct paid;
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for paid {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "paid")
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for paid {
                #[inline]
                fn clone(&self) -> paid {
                    *self
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for paid {}
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for paid {
                    type QueryId = paid;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            };
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for paid {
                #[inline]
                fn default() -> paid {
                    paid {}
                }
            }
            impl diesel::expression::Expression for paid {
                type SqlType = Bool;
            }
            impl<DB> diesel::query_builder::QueryFragment<DB> for paid
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
                    __diesel_internal_out.push_identifier("paid")
                }
            }
            impl diesel::SelectableExpression<super::table> for paid {}
            impl<QS> diesel::AppearsOnTable<QS> for paid
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
            > for paid
            where
                paid: diesel::AppearsOnTable<
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
            > for paid
            where
                paid: diesel::AppearsOnTable<
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
            > for paid
            where
                paid: diesel::SelectableExpression<Join>
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
            > for paid
            where
                From: diesel::query_source::QuerySource,
                paid: diesel::SelectableExpression<From>
                    + diesel::AppearsOnTable<
                        diesel::internal::table_macro::SelectStatement<
                            diesel::internal::table_macro::FromClause<From>,
                        >,
                    >,
            {}
            impl<__GB> diesel::expression::ValidGrouping<__GB> for paid
            where
                __GB: diesel::expression::IsContainedInGroupBy<
                    paid,
                    Output = diesel::expression::is_contained_in_group_by::Yes,
                >,
            {
                type IsAggregate = diesel::expression::is_aggregate::Yes;
            }
            impl diesel::expression::ValidGrouping<()> for paid {
                type IsAggregate = diesel::expression::is_aggregate::No;
            }
            impl diesel::expression::IsContainedInGroupBy<paid> for paid {
                type Output = diesel::expression::is_contained_in_group_by::Yes;
            }
            impl diesel::query_source::Column for paid {
                type Table = super::table;
                const NAME: &'static str = "paid";
            }
            impl<T> diesel::EqAll<T> for paid
            where
                T: diesel::expression::AsExpression<Bool>,
                diesel::dsl::Eq<
                    paid,
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
            > for paid {
                type Count = diesel::query_source::Once;
            }
            impl diesel::SelectableExpression<diesel::query_builder::Only<super::table>>
            for paid {}
            impl<
                TSM,
            > diesel::query_source::AppearsInFromClause<
                diesel::query_builder::Tablesample<super::table, TSM>,
            > for paid
            where
                TSM: diesel::internal::table_macro::TablesampleMethod,
            {
                type Count = diesel::query_source::Once;
            }
            impl<
                TSM,
            > diesel::SelectableExpression<
                diesel::query_builder::Tablesample<super::table, TSM>,
            > for paid
            where
                TSM: diesel::internal::table_macro::TablesampleMethod,
            {}
            #[allow(non_camel_case_types, dead_code)]
            pub struct production_date;
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for production_date {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "production_date")
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for production_date {
                #[inline]
                fn clone(&self) -> production_date {
                    *self
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for production_date {}
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for production_date {
                    type QueryId = production_date;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            };
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for production_date {
                #[inline]
                fn default() -> production_date {
                    production_date {}
                }
            }
            impl diesel::expression::Expression for production_date {
                type SqlType = Date;
            }
            impl<DB> diesel::query_builder::QueryFragment<DB> for production_date
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
                    __diesel_internal_out.push_identifier("production_date")
                }
            }
            impl diesel::SelectableExpression<super::table> for production_date {}
            impl<QS> diesel::AppearsOnTable<QS> for production_date
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
            > for production_date
            where
                production_date: diesel::AppearsOnTable<
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
            > for production_date
            where
                production_date: diesel::AppearsOnTable<
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
            > for production_date
            where
                production_date: diesel::SelectableExpression<Join>
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
            > for production_date
            where
                From: diesel::query_source::QuerySource,
                production_date: diesel::SelectableExpression<From>
                    + diesel::AppearsOnTable<
                        diesel::internal::table_macro::SelectStatement<
                            diesel::internal::table_macro::FromClause<From>,
                        >,
                    >,
            {}
            impl<__GB> diesel::expression::ValidGrouping<__GB> for production_date
            where
                __GB: diesel::expression::IsContainedInGroupBy<
                    production_date,
                    Output = diesel::expression::is_contained_in_group_by::Yes,
                >,
            {
                type IsAggregate = diesel::expression::is_aggregate::Yes;
            }
            impl diesel::expression::ValidGrouping<()> for production_date {
                type IsAggregate = diesel::expression::is_aggregate::No;
            }
            impl diesel::expression::IsContainedInGroupBy<production_date>
            for production_date {
                type Output = diesel::expression::is_contained_in_group_by::Yes;
            }
            impl diesel::query_source::Column for production_date {
                type Table = super::table;
                const NAME: &'static str = "production_date";
            }
            impl<T> diesel::EqAll<T> for production_date
            where
                T: diesel::expression::AsExpression<Date>,
                diesel::dsl::Eq<
                    production_date,
                    T::Expression,
                >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
            {
                type Output = diesel::dsl::Eq<Self, T::Expression>;
                fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                    use diesel::expression_methods::ExpressionMethods;
                    self.eq(__diesel_internal_rhs)
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for production_date
            where
                Rhs: diesel::expression::AsExpression<
                    <<production_date as diesel::Expression>::SqlType as diesel::sql_types::ops::Add>::Rhs,
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
            impl<Rhs> ::std::ops::Sub<Rhs> for production_date
            where
                Rhs: diesel::expression::AsExpression<
                    <<production_date as diesel::Expression>::SqlType as diesel::sql_types::ops::Sub>::Rhs,
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
            impl diesel::query_source::AppearsInFromClause<
                diesel::query_builder::Only<super::table>,
            > for production_date {
                type Count = diesel::query_source::Once;
            }
            impl diesel::SelectableExpression<diesel::query_builder::Only<super::table>>
            for production_date {}
            impl<
                TSM,
            > diesel::query_source::AppearsInFromClause<
                diesel::query_builder::Tablesample<super::table, TSM>,
            > for production_date
            where
                TSM: diesel::internal::table_macro::TablesampleMethod,
            {
                type Count = diesel::query_source::Once;
            }
            impl<
                TSM,
            > diesel::SelectableExpression<
                diesel::query_builder::Tablesample<super::table, TSM>,
            > for production_date
            where
                TSM: diesel::internal::table_macro::TablesampleMethod,
            {}
            #[allow(non_camel_case_types, dead_code)]
            pub struct taken_date;
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for taken_date {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "taken_date")
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for taken_date {
                #[inline]
                fn clone(&self) -> taken_date {
                    *self
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for taken_date {}
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for taken_date {
                    type QueryId = taken_date;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            };
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for taken_date {
                #[inline]
                fn default() -> taken_date {
                    taken_date {}
                }
            }
            impl diesel::expression::Expression for taken_date {
                type SqlType = Date;
            }
            impl<DB> diesel::query_builder::QueryFragment<DB> for taken_date
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
                    __diesel_internal_out.push_identifier("taken_date")
                }
            }
            impl diesel::SelectableExpression<super::table> for taken_date {}
            impl<QS> diesel::AppearsOnTable<QS> for taken_date
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
            > for taken_date
            where
                taken_date: diesel::AppearsOnTable<
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
            > for taken_date
            where
                taken_date: diesel::AppearsOnTable<
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
            > for taken_date
            where
                taken_date: diesel::SelectableExpression<Join>
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
            > for taken_date
            where
                From: diesel::query_source::QuerySource,
                taken_date: diesel::SelectableExpression<From>
                    + diesel::AppearsOnTable<
                        diesel::internal::table_macro::SelectStatement<
                            diesel::internal::table_macro::FromClause<From>,
                        >,
                    >,
            {}
            impl<__GB> diesel::expression::ValidGrouping<__GB> for taken_date
            where
                __GB: diesel::expression::IsContainedInGroupBy<
                    taken_date,
                    Output = diesel::expression::is_contained_in_group_by::Yes,
                >,
            {
                type IsAggregate = diesel::expression::is_aggregate::Yes;
            }
            impl diesel::expression::ValidGrouping<()> for taken_date {
                type IsAggregate = diesel::expression::is_aggregate::No;
            }
            impl diesel::expression::IsContainedInGroupBy<taken_date> for taken_date {
                type Output = diesel::expression::is_contained_in_group_by::Yes;
            }
            impl diesel::query_source::Column for taken_date {
                type Table = super::table;
                const NAME: &'static str = "taken_date";
            }
            impl<T> diesel::EqAll<T> for taken_date
            where
                T: diesel::expression::AsExpression<Date>,
                diesel::dsl::Eq<
                    taken_date,
                    T::Expression,
                >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
            {
                type Output = diesel::dsl::Eq<Self, T::Expression>;
                fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                    use diesel::expression_methods::ExpressionMethods;
                    self.eq(__diesel_internal_rhs)
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for taken_date
            where
                Rhs: diesel::expression::AsExpression<
                    <<taken_date as diesel::Expression>::SqlType as diesel::sql_types::ops::Add>::Rhs,
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
            impl<Rhs> ::std::ops::Sub<Rhs> for taken_date
            where
                Rhs: diesel::expression::AsExpression<
                    <<taken_date as diesel::Expression>::SqlType as diesel::sql_types::ops::Sub>::Rhs,
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
            impl diesel::query_source::AppearsInFromClause<
                diesel::query_builder::Only<super::table>,
            > for taken_date {
                type Count = diesel::query_source::Once;
            }
            impl diesel::SelectableExpression<diesel::query_builder::Only<super::table>>
            for taken_date {}
            impl<
                TSM,
            > diesel::query_source::AppearsInFromClause<
                diesel::query_builder::Tablesample<super::table, TSM>,
            > for taken_date
            where
                TSM: diesel::internal::table_macro::TablesampleMethod,
            {
                type Count = diesel::query_source::Once;
            }
            impl<
                TSM,
            > diesel::SelectableExpression<
                diesel::query_builder::Tablesample<super::table, TSM>,
            > for taken_date
            where
                TSM: diesel::internal::table_macro::TablesampleMethod,
            {}
            #[allow(non_camel_case_types, dead_code)]
            pub struct price;
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for price {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "price")
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for price {
                #[inline]
                fn clone(&self) -> price {
                    *self
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for price {}
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for price {
                    type QueryId = price;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            };
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for price {
                #[inline]
                fn default() -> price {
                    price {}
                }
            }
            impl diesel::expression::Expression for price {
                type SqlType = Double;
            }
            impl<DB> diesel::query_builder::QueryFragment<DB> for price
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
                    __diesel_internal_out.push_identifier("price")
                }
            }
            impl diesel::SelectableExpression<super::table> for price {}
            impl<QS> diesel::AppearsOnTable<QS> for price
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
            > for price
            where
                price: diesel::AppearsOnTable<
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
            > for price
            where
                price: diesel::AppearsOnTable<
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
            > for price
            where
                price: diesel::SelectableExpression<Join>
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
            > for price
            where
                From: diesel::query_source::QuerySource,
                price: diesel::SelectableExpression<From>
                    + diesel::AppearsOnTable<
                        diesel::internal::table_macro::SelectStatement<
                            diesel::internal::table_macro::FromClause<From>,
                        >,
                    >,
            {}
            impl<__GB> diesel::expression::ValidGrouping<__GB> for price
            where
                __GB: diesel::expression::IsContainedInGroupBy<
                    price,
                    Output = diesel::expression::is_contained_in_group_by::Yes,
                >,
            {
                type IsAggregate = diesel::expression::is_aggregate::Yes;
            }
            impl diesel::expression::ValidGrouping<()> for price {
                type IsAggregate = diesel::expression::is_aggregate::No;
            }
            impl diesel::expression::IsContainedInGroupBy<price> for price {
                type Output = diesel::expression::is_contained_in_group_by::Yes;
            }
            impl diesel::query_source::Column for price {
                type Table = super::table;
                const NAME: &'static str = "price";
            }
            impl<T> diesel::EqAll<T> for price
            where
                T: diesel::expression::AsExpression<Double>,
                diesel::dsl::Eq<
                    price,
                    T::Expression,
                >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
            {
                type Output = diesel::dsl::Eq<Self, T::Expression>;
                fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                    use diesel::expression_methods::ExpressionMethods;
                    self.eq(__diesel_internal_rhs)
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for price
            where
                Rhs: diesel::expression::AsExpression<
                    <<price as diesel::Expression>::SqlType as diesel::sql_types::ops::Add>::Rhs,
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
            impl<Rhs> ::std::ops::Sub<Rhs> for price
            where
                Rhs: diesel::expression::AsExpression<
                    <<price as diesel::Expression>::SqlType as diesel::sql_types::ops::Sub>::Rhs,
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
            impl<Rhs> ::std::ops::Div<Rhs> for price
            where
                Rhs: diesel::expression::AsExpression<
                    <<price as diesel::Expression>::SqlType as diesel::sql_types::ops::Div>::Rhs,
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
            impl<Rhs> ::std::ops::Mul<Rhs> for price
            where
                Rhs: diesel::expression::AsExpression<
                    <<price as diesel::Expression>::SqlType as diesel::sql_types::ops::Mul>::Rhs,
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
            > for price {
                type Count = diesel::query_source::Once;
            }
            impl diesel::SelectableExpression<diesel::query_builder::Only<super::table>>
            for price {}
            impl<
                TSM,
            > diesel::query_source::AppearsInFromClause<
                diesel::query_builder::Tablesample<super::table, TSM>,
            > for price
            where
                TSM: diesel::internal::table_macro::TablesampleMethod,
            {
                type Count = diesel::query_source::Once;
            }
            impl<
                TSM,
            > diesel::SelectableExpression<
                diesel::query_builder::Tablesample<super::table, TSM>,
            > for price
            where
                TSM: diesel::internal::table_macro::TablesampleMethod,
            {}
            #[allow(non_camel_case_types, dead_code)]
            pub struct amount;
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::fmt::Debug for amount {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "amount")
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::clone::Clone for amount {
                #[inline]
                fn clone(&self) -> amount {
                    *self
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::marker::Copy for amount {}
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for amount {
                    type QueryId = amount;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            };
            #[automatically_derived]
            #[allow(non_camel_case_types, dead_code)]
            impl ::core::default::Default for amount {
                #[inline]
                fn default() -> amount {
                    amount {}
                }
            }
            impl diesel::expression::Expression for amount {
                type SqlType = Integer;
            }
            impl<DB> diesel::query_builder::QueryFragment<DB> for amount
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
                    __diesel_internal_out.push_identifier("amount")
                }
            }
            impl diesel::SelectableExpression<super::table> for amount {}
            impl<QS> diesel::AppearsOnTable<QS> for amount
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
            > for amount
            where
                amount: diesel::AppearsOnTable<
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
            > for amount
            where
                amount: diesel::AppearsOnTable<
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
            > for amount
            where
                amount: diesel::SelectableExpression<Join>
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
            > for amount
            where
                From: diesel::query_source::QuerySource,
                amount: diesel::SelectableExpression<From>
                    + diesel::AppearsOnTable<
                        diesel::internal::table_macro::SelectStatement<
                            diesel::internal::table_macro::FromClause<From>,
                        >,
                    >,
            {}
            impl<__GB> diesel::expression::ValidGrouping<__GB> for amount
            where
                __GB: diesel::expression::IsContainedInGroupBy<
                    amount,
                    Output = diesel::expression::is_contained_in_group_by::Yes,
                >,
            {
                type IsAggregate = diesel::expression::is_aggregate::Yes;
            }
            impl diesel::expression::ValidGrouping<()> for amount {
                type IsAggregate = diesel::expression::is_aggregate::No;
            }
            impl diesel::expression::IsContainedInGroupBy<amount> for amount {
                type Output = diesel::expression::is_contained_in_group_by::Yes;
            }
            impl diesel::query_source::Column for amount {
                type Table = super::table;
                const NAME: &'static str = "amount";
            }
            impl<T> diesel::EqAll<T> for amount
            where
                T: diesel::expression::AsExpression<Integer>,
                diesel::dsl::Eq<
                    amount,
                    T::Expression,
                >: diesel::Expression<SqlType = diesel::sql_types::Bool>,
            {
                type Output = diesel::dsl::Eq<Self, T::Expression>;
                fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                    use diesel::expression_methods::ExpressionMethods;
                    self.eq(__diesel_internal_rhs)
                }
            }
            impl<Rhs> ::std::ops::Add<Rhs> for amount
            where
                Rhs: diesel::expression::AsExpression<
                    <<amount as diesel::Expression>::SqlType as diesel::sql_types::ops::Add>::Rhs,
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
            impl<Rhs> ::std::ops::Sub<Rhs> for amount
            where
                Rhs: diesel::expression::AsExpression<
                    <<amount as diesel::Expression>::SqlType as diesel::sql_types::ops::Sub>::Rhs,
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
            impl<Rhs> ::std::ops::Div<Rhs> for amount
            where
                Rhs: diesel::expression::AsExpression<
                    <<amount as diesel::Expression>::SqlType as diesel::sql_types::ops::Div>::Rhs,
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
            impl<Rhs> ::std::ops::Mul<Rhs> for amount
            where
                Rhs: diesel::expression::AsExpression<
                    <<amount as diesel::Expression>::SqlType as diesel::sql_types::ops::Mul>::Rhs,
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
            > for amount {
                type Count = diesel::query_source::Once;
            }
            impl diesel::SelectableExpression<diesel::query_builder::Only<super::table>>
            for amount {}
            impl<
                TSM,
            > diesel::query_source::AppearsInFromClause<
                diesel::query_builder::Tablesample<super::table, TSM>,
            > for amount
            where
                TSM: diesel::internal::table_macro::TablesampleMethod,
            {
                type Count = diesel::query_source::Once;
            }
            impl<
                TSM,
            > diesel::SelectableExpression<
                diesel::query_builder::Tablesample<super::table, TSM>,
            > for amount
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
            impl diesel::expression::IsContainedInGroupBy<id> for user_id {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<user_id> for id {
                type Output = diesel::expression::is_contained_in_group_by::Yes;
            }
            impl diesel::expression::IsContainedInGroupBy<id> for paid {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<paid> for id {
                type Output = diesel::expression::is_contained_in_group_by::Yes;
            }
            impl diesel::expression::IsContainedInGroupBy<id> for production_date {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<production_date> for id {
                type Output = diesel::expression::is_contained_in_group_by::Yes;
            }
            impl diesel::expression::IsContainedInGroupBy<id> for taken_date {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<taken_date> for id {
                type Output = diesel::expression::is_contained_in_group_by::Yes;
            }
            impl diesel::expression::IsContainedInGroupBy<id> for price {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<price> for id {
                type Output = diesel::expression::is_contained_in_group_by::Yes;
            }
            impl diesel::expression::IsContainedInGroupBy<id> for amount {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<amount> for id {
                type Output = diesel::expression::is_contained_in_group_by::Yes;
            }
            impl diesel::expression::IsContainedInGroupBy<id> for description {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<description> for id {
                type Output = diesel::expression::is_contained_in_group_by::Yes;
            }
            impl diesel::expression::IsContainedInGroupBy<user_id> for paid {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<paid> for user_id {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<user_id> for production_date {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<production_date> for user_id {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<user_id> for taken_date {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<taken_date> for user_id {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<user_id> for price {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<price> for user_id {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<user_id> for amount {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<amount> for user_id {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<user_id> for description {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<description> for user_id {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<paid> for production_date {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<production_date> for paid {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<paid> for taken_date {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<taken_date> for paid {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<paid> for price {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<price> for paid {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<paid> for amount {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<amount> for paid {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<paid> for description {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<description> for paid {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<production_date>
            for taken_date {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<taken_date>
            for production_date {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<production_date> for price {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<price> for production_date {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<production_date> for amount {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<amount> for production_date {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<production_date>
            for description {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<description>
            for production_date {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<taken_date> for price {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<price> for taken_date {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<taken_date> for amount {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<amount> for taken_date {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<taken_date> for description {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<description> for taken_date {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<price> for amount {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<amount> for price {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<price> for description {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<description> for price {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<amount> for description {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
            impl diesel::expression::IsContainedInGroupBy<description> for amount {
                type Output = diesel::expression::is_contained_in_group_by::No;
            }
        }
    }
    #[allow(unused_imports)]
    const _: () = {
        use diesel;
        use diesel::insertable::Insertable;
        use diesel::internal::derives::insertable::UndecoratedInsertRecord;
        use diesel::prelude::*;
        #[allow(unused_qualifications)]
        impl Insertable<product_table::table> for Product {
            type Values = <(
                std::option::Option<diesel::dsl::Eq<product_table::id, i32>>,
                std::option::Option<diesel::dsl::Eq<product_table::user_id, i32>>,
                std::option::Option<diesel::dsl::Eq<product_table::paid, bool>>,
                std::option::Option<
                    diesel::dsl::Eq<product_table::production_date, NaiveDate>,
                >,
                std::option::Option<
                    diesel::dsl::Eq<product_table::taken_date, NaiveDate>,
                >,
                std::option::Option<diesel::dsl::Eq<product_table::price, f64>>,
                std::option::Option<diesel::dsl::Eq<product_table::amount, i32>>,
                std::option::Option<diesel::dsl::Eq<product_table::description, String>>,
            ) as Insertable<product_table::table>>::Values;
            fn values(
                self,
            ) -> <(
                std::option::Option<diesel::dsl::Eq<product_table::id, i32>>,
                std::option::Option<diesel::dsl::Eq<product_table::user_id, i32>>,
                std::option::Option<diesel::dsl::Eq<product_table::paid, bool>>,
                std::option::Option<
                    diesel::dsl::Eq<product_table::production_date, NaiveDate>,
                >,
                std::option::Option<
                    diesel::dsl::Eq<product_table::taken_date, NaiveDate>,
                >,
                std::option::Option<diesel::dsl::Eq<product_table::price, f64>>,
                std::option::Option<diesel::dsl::Eq<product_table::amount, i32>>,
                std::option::Option<diesel::dsl::Eq<product_table::description, String>>,
            ) as Insertable<product_table::table>>::Values {
                (
                    std::option::Option::Some(product_table::id.eq(self.id)),
                    std::option::Option::Some(product_table::user_id.eq(self.user_id)),
                    std::option::Option::Some(product_table::paid.eq(self.paid)),
                    std::option::Option::Some(
                        product_table::production_date.eq(self.production_date),
                    ),
                    std::option::Option::Some(
                        product_table::taken_date.eq(self.taken_date),
                    ),
                    std::option::Option::Some(product_table::price.eq(self.price)),
                    std::option::Option::Some(product_table::amount.eq(self.amount)),
                    std::option::Option::Some(
                        product_table::description.eq(self.description),
                    ),
                )
                    .values()
            }
        }
        #[allow(unused_qualifications)]
        impl<'insert> Insertable<product_table::table> for &'insert Product {
            type Values = <(
                std::option::Option<diesel::dsl::Eq<product_table::id, &'insert i32>>,
                std::option::Option<
                    diesel::dsl::Eq<product_table::user_id, &'insert i32>,
                >,
                std::option::Option<diesel::dsl::Eq<product_table::paid, &'insert bool>>,
                std::option::Option<
                    diesel::dsl::Eq<product_table::production_date, &'insert NaiveDate>,
                >,
                std::option::Option<
                    diesel::dsl::Eq<product_table::taken_date, &'insert NaiveDate>,
                >,
                std::option::Option<diesel::dsl::Eq<product_table::price, &'insert f64>>,
                std::option::Option<
                    diesel::dsl::Eq<product_table::amount, &'insert i32>,
                >,
                std::option::Option<
                    diesel::dsl::Eq<product_table::description, &'insert String>,
                >,
            ) as Insertable<product_table::table>>::Values;
            fn values(
                self,
            ) -> <(
                std::option::Option<diesel::dsl::Eq<product_table::id, &'insert i32>>,
                std::option::Option<
                    diesel::dsl::Eq<product_table::user_id, &'insert i32>,
                >,
                std::option::Option<diesel::dsl::Eq<product_table::paid, &'insert bool>>,
                std::option::Option<
                    diesel::dsl::Eq<product_table::production_date, &'insert NaiveDate>,
                >,
                std::option::Option<
                    diesel::dsl::Eq<product_table::taken_date, &'insert NaiveDate>,
                >,
                std::option::Option<diesel::dsl::Eq<product_table::price, &'insert f64>>,
                std::option::Option<
                    diesel::dsl::Eq<product_table::amount, &'insert i32>,
                >,
                std::option::Option<
                    diesel::dsl::Eq<product_table::description, &'insert String>,
                >,
            ) as Insertable<product_table::table>>::Values {
                (
                    std::option::Option::Some(product_table::id.eq(&self.id)),
                    std::option::Option::Some(product_table::user_id.eq(&self.user_id)),
                    std::option::Option::Some(product_table::paid.eq(&self.paid)),
                    std::option::Option::Some(
                        product_table::production_date.eq(&self.production_date),
                    ),
                    std::option::Option::Some(
                        product_table::taken_date.eq(&self.taken_date),
                    ),
                    std::option::Option::Some(product_table::price.eq(&self.price)),
                    std::option::Option::Some(product_table::amount.eq(&self.amount)),
                    std::option::Option::Some(
                        product_table::description.eq(&self.description),
                    ),
                )
                    .values()
            }
        }
        impl UndecoratedInsertRecord<product_table::table> for Product {}
    };
    #[allow(unused_imports)]
    const _: () = {
        use diesel;
        use diesel::expression::Selectable;
        impl<__DB: diesel::backend::Backend> Selectable<__DB> for Product {
            type SelectExpression = (
                product_table::id,
                product_table::user_id,
                product_table::paid,
                product_table::production_date,
                product_table::taken_date,
                product_table::price,
                product_table::amount,
                product_table::description,
            );
            fn construct_selection() -> Self::SelectExpression {
                (
                    product_table::id,
                    product_table::user_id,
                    product_table::paid,
                    product_table::production_date,
                    product_table::taken_date,
                    product_table::price,
                    product_table::amount,
                    product_table::description,
                )
            }
        }
    };
    impl Migrationable for Product {
        fn get_up_migration(builder: impl SchemaBuilder) -> String {
            Table::create()
                .table(ProductTable::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(ProductTable::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(ColumnDef::new(ProductTable::UserId).integer())
                .col(ColumnDef::new(ProductTable::Paid).boolean())
                .col(ColumnDef::new(ProductTable::ProductionDate).date_time())
                .col(ColumnDef::new(ProductTable::TakenDate).date_time())
                .col(ColumnDef::new(ProductTable::Price).double())
                .col(ColumnDef::new(ProductTable::Amount).integer())
                .col(ColumnDef::new(ProductTable::Description).text())
                .build(builder)
        }
        fn get_down_migration(builder: impl SchemaBuilder) -> String {
            Table::drop().table(ProductTable::Table).build(builder)
        }
    }
}
