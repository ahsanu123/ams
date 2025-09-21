use super::get_sql_connection_trait::GetSqlConnectionTrait;
use ams_entity::prelude::*;
use sea_orm::{
    prelude::async_trait::async_trait, ActiveModelTrait, DbErr, EntityTrait, IntoActiveModel,
    PrimaryKeyTrait,
};

// TODO: change TEntity::Model into TEntity::ActiveModel for inserting or updating

#[async_trait]
pub trait AbstractRepository<TEntity>
where
    TEntity: EntityTrait + GetSqlConnectionTrait,
{
    type PrimaryKeyType: Send;

    async fn get_all() -> Result<Vec<TEntity::Model>, DbErr>;
    async fn get_by_id(id: Self::PrimaryKeyType) -> Result<Option<TEntity::Model>, DbErr>;
    async fn update(model: TEntity::Model) -> Result<TEntity::Model, DbErr>;
    async fn create(model: TEntity::ActiveModel) -> Result<TEntity::Model, DbErr>;
    async fn delete(id: Self::PrimaryKeyType) -> Result<u32, DbErr>;
}

#[async_trait]
impl<TEntity> AbstractRepository<TEntity> for TEntity
where
    TEntity: EntityTrait + GetSqlConnectionTrait,
    TEntity::Model: IntoActiveModel<TEntity::ActiveModel>,
    TEntity::Model: Into<TEntity::ActiveModel>,
    TEntity::ActiveModel: Send,
{
    type PrimaryKeyType = <TEntity::PrimaryKey as PrimaryKeyTrait>::ValueType;

    async fn get_by_id(id: Self::PrimaryKeyType) -> Result<Option<TEntity::Model>, DbErr> {
        let conn = TEntity::get_connection().await;

        TEntity::find_by_id(id).one(conn).await
    }
    async fn update(model: TEntity::Model) -> Result<TEntity::Model, DbErr> {
        let conn = TEntity::get_connection().await;

        let active_model: TEntity::ActiveModel = model.into();
        let updated_model: TEntity::Model = active_model.update(conn).await?;

        Ok(updated_model)
    }

    async fn create(model: TEntity::ActiveModel) -> Result<TEntity::Model, DbErr> {
        let conn = TEntity::get_connection().await;

        let inserted_model: TEntity::Model = model.insert(conn).await?;

        Ok(inserted_model)
    }
    async fn delete(id: Self::PrimaryKeyType) -> Result<u32, DbErr> {
        todo!()
    }
    async fn get_all() -> Result<Vec<TEntity::Model>, DbErr> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::helper::ENV_VAR;
    use ams_entity::user_table;
    use chrono::Local;
    use sea_orm::{ActiveValue::Set, Database, DatabaseConnection};

    #[tokio::test]
    async fn create_sea_orm_connection() {
        let _: DatabaseConnection = Database::connect(ENV_VAR.sqlite_connection_string.clone())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn get_by_id() {
        let may_user = UserTable::get_by_id(1).await.unwrap();

        if let Some(user) = may_user {
            println!("Found some user: {user:?}");
        } else {
            println!("cant find user");
        }
    }

    #[tokio::test]
    async fn create_new_user() {
        let new_user = user_table::ActiveModel {
            username: Set("John Thompson".to_owned()),
            is_active: Set(true),
            is_admin: Set(false),
            created_date: Set(Local::now().naive_local()),
            updated_date: Set(Local::now().naive_local()),
            ..Default::default()
        };

        let result = UserTable::create(new_user).await.unwrap();

        println!("inserted id {0}", result.username);
    }
}
