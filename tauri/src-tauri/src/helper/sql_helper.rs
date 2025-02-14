use sea_query::{
    ColumnDef, Expr, Func, Iden, InsertStatement, OnConflict, Order, PostgresQueryBuilder, Query,
    QueryBuilder, QueryStatementBuilder, SqliteQueryBuilder, Table,
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
}

pub struct SqlHelper {
    pub db_pool: Pool<Database>,
    pub sql_connection: SqlConnection,
}

impl SqlHelper {
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

    pub fn get_builder(database_type: DatabaseType) {
        match database_type {
            DatabaseType::Postgresql => PostgresQueryBuilder,
            DatabaseType::Sqlite => SqliteQueryBuilder,
        }
    }
}

impl SqlxHelper for SqlHelper {
    async fn execute_query_async(&self, statement: impl SqlxBinder) {
        let (sql, value) =
            statement.build_sqlx(Self::get_builder(self.sql_connection.database_type));

        // TODO: Not Complete yet
        let row = sqlx::query_with(&sql, value)
            .execute(&self.db_pool)
            .await
            .unwrap();
    }

    async fn query_async<T>(&self, statement: impl SqlxBinder) -> T {
        todo!()
    }
}

impl Default for SqlHelper {
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
