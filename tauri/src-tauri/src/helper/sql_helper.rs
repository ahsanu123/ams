use sea_query::{
    ColumnDef, Expr, Func, Iden, InsertStatement, OnConflict, Order, PostgresQueryBuilder, Query,
    QueryBuilder, QueryStatementBuilder, SchemaStatementBuilder, SqliteQueryBuilder, Table,
    TableCreateStatement,
};
use sea_query::{QueryStatementWriter, SqliteQueryBuilder};
use sea_query_binder::SqlxBinder;
use sqlx::{Database, Pool, SqlitePool};

pub enum DatabaseType {
    Postgresql,
    Sqlite,
}

pub struct SqlConnection {
    pub connection_string: String,
    pub database_type: DatabaseType,
}

pub trait SqlxHelper {
    async fn execute_query_async(&self, statement: impl SqlxBinder);
    async fn query_async<T>(&self, statement: impl SqlxBinder) -> T;
    async fn query_single_async<T>(&self, statement: impl SqlxBinder) -> Option<T>;
    async fn migrate_table(&self, table_statement: impl SchemaStatementBuilder);
}

pub struct SqlConnectionProvider {
    pub db_pool: Pool<Database>,
    pub sql_connection: SqlConnection,
}

impl SqlConnectionProvider {
    pub async fn create_connection(&self, sql_connection: SqlConnection) -> Pool<Database> {
        match sql_connection.database_type {
            DatabaseType::Postgresql => todo!(),
            DatabaseType::Sqlite => {
                self.db_pool = SqlitePool::connect(&sql_connection.connection_string)
                    .await
                    .unwrap();

                self.db_pool
            }
        }
    }

    pub fn get_builder(&self) {
        match self.sql_connection.database_type {
            DatabaseType::Postgresql => PostgresQueryBuilder,
            DatabaseType::Sqlite => SqliteQueryBuilder,
        }
    }
}

impl SqlxHelper for SqlConnectionProvider {
    async fn execute_query_async(&self, statement: impl SqlxBinder) {
        let (sql, value) = statement.build_sqlx(self.get_builder());

        // TODO: Not Complete yet
        let row = sqlx::query_with(&sql, value)
            .execute(&self.db_pool)
            .await
            .unwrap();
    }

    async fn query_async<T>(&self, statement: impl SqlxBinder) -> T {
        todo!()
    }

    async fn query_single_async<T>(&self, statement: impl SqlxBinder) -> Option<T> {
        todo!()
    }

    async fn migrate_table(&self, table_statement: impl SchemaStatementBuilder) {
        let buider = self.get_builder();
        let sql = table_statement.build(builder);

        sqlx::query(&sql).execute(self.db_pool).await;
    }
}

impl Default for SqlConnectionProvider {
    fn default() -> Self {
        let default_sql_connection = SqlConnection {
            connection_string: "sqlite::memory:".into(),
            database_type: DatabaseType::Sqlite,
        };

        Self {
            db_pool: Self::create_connection(Self, default_sql_connection),
            sql_connection: default_sql_connection,
        }
    }
}
