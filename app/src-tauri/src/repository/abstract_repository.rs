use super::database_connection::DATABASE_CONNECTION_POOL;
use ams_entity::prelude::*;
use sea_orm::{Database, DatabaseConnection, EntityTrait, PrimaryKeyTrait};

pub trait GetSqlConnectionTrait {
    async fn get_connection(self) -> DatabaseConnection;
}

pub trait AbstractRepository<TEntity>
where
    TEntity: EntityTrait + GetSqlConnectionTrait,
{
    type PrimaryKeyType: Send;

    async fn get_by_id(
        self,
        id: Self::PrimaryKeyType,
    ) -> Result<Option<TEntity::Model>, sea_orm::DbErr>;
}

impl<TEntity> GetSqlConnectionTrait for TEntity
where
    TEntity: EntityTrait,
{
    async fn get_connection(self) -> DatabaseConnection {
        DATABASE_CONNECTION_POOL.clone()
    }
}

impl<TEntity> AbstractRepository<TEntity> for UserTable
where
    TEntity: EntityTrait + GetSqlConnectionTrait,
{
    type PrimaryKeyType = <TEntity::PrimaryKey as PrimaryKeyTrait>::ValueType;

    async fn get_by_id(
        self,
        id: Self::PrimaryKeyType,
    ) -> Result<Option<<TEntity as EntityTrait>::Model>, sea_orm::DbErr> {
        let conn = &mut self.get_connection().await;

        TEntity::find_by_id(id).one(conn).await
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::helper::ENV_VAR;
    use sea_orm::{Database, DatabaseConnection, EntityTrait};

    #[tokio::test]
    async fn create_sea_orm_connection() {
        let db: DatabaseConnection = Database::connect(ENV_VAR.sqlite_connection_string.clone())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn abstract_repo_get_by_id() {
        let db: DatabaseConnection = Database::connect(ENV_VAR.sqlite_connection_string.clone())
            .await
            .unwrap();

        let may_user = UserTable::find_by_id(1).one(&db).await.unwrap();

        if let Some(user) = may_user {
            println!("Found some user");
        } else {
            println!("cant find user");
        }
    }
}
