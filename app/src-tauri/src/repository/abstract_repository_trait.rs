use super::get_sql_connection_trait::GetSqlConnectionTrait;
use sea_orm::{
    prelude::async_trait::async_trait, ActiveModelTrait, DbErr, EntityTrait, IntoActiveModel,
    PrimaryKeyTrait,
};

#[async_trait]
pub trait AbstractRepository<TEntity>
where
    TEntity: EntityTrait + GetSqlConnectionTrait,
{
    type PrimaryKeyType: Send;

    async fn get_all() -> Result<Vec<TEntity::Model>, DbErr>;
    async fn get_by_id(id: Self::PrimaryKeyType) -> Result<Option<TEntity::Model>, DbErr>;
    async fn update_by_model(model: TEntity::ActiveModel) -> Result<TEntity::Model, DbErr>;
    async fn create(model: TEntity::ActiveModel) -> Result<TEntity::Model, DbErr>;
    async fn delete_by_model_id(id: Self::PrimaryKeyType) -> Result<u64, DbErr>;
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
    async fn update_by_model(model: TEntity::ActiveModel) -> Result<TEntity::Model, DbErr> {
        let conn = TEntity::get_connection().await;

        let updated_model: TEntity::Model = model.update(conn).await?;
        Ok(updated_model)
    }

    async fn create(model: TEntity::ActiveModel) -> Result<TEntity::Model, DbErr> {
        let conn = TEntity::get_connection().await;

        let inserted_model: TEntity::Model = model.insert(conn).await?;
        Ok(inserted_model)
    }
    async fn delete_by_model_id(id: Self::PrimaryKeyType) -> Result<u64, DbErr> {
        let conn = TEntity::get_connection().await;

        let deleted_result = TEntity::delete_by_id(id).exec(conn).await.unwrap();
        Ok(deleted_result.rows_affected)
    }
    async fn get_all() -> Result<Vec<TEntity::Model>, DbErr> {
        let conn = TEntity::get_connection().await;

        let all_data = TEntity::find().all(conn).await.unwrap();
        Ok(all_data)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::helper::ENV_VAR;
    use ams_entity::prelude::*;
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

    #[tokio::test]
    async fn update_user_data() {
        let john = UserTable::get_by_id(1).await.unwrap();

        if let Some(data) = john {
            let mut active_data: user_table::ActiveModel = data.into();
            active_data.username = Set("Honduras".to_owned());

            let update_result = UserTable::update_by_model(active_data).await.unwrap();
            println!("updated data: {update_result:?}");
        } else {
            println!("cant get user data");
        }
    }

    #[tokio::test]
    async fn get_all() {
        let all_data = UserTable::get_all().await.unwrap();

        assert!(!all_data.is_empty());

        let first_data = all_data.first();
        if let Some(data) = first_data {
            println!("first data is: {data:?}");
        }
    }

    #[tokio::test]
    async fn delete_user() {
        let deleted_count = UserTable::delete_by_model_id(2).await.unwrap();

        assert!(deleted_count > 0);
    }
}
