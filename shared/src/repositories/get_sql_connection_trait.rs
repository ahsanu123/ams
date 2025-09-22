use super::database_connection::get_database_connection;
use sea_orm::{DatabaseConnection, EntityTrait, prelude::async_trait::async_trait};

#[async_trait]
pub trait GetSqlConnectionTrait {
    async fn get_connection() -> &'static DatabaseConnection;
}

#[async_trait]
impl<TEntity> GetSqlConnectionTrait for TEntity
where
    TEntity: EntityTrait,
{
    async fn get_connection() -> &'static DatabaseConnection {
        get_database_connection().await
    }
}
