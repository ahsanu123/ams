use crate::repositories::database_connection::get_database_connection;
use sea_orm::{DbErr, entity::*, prelude::async_trait::async_trait};

#[async_trait]
pub trait GenericCrudRepositoryWithRelation<T, R>
where
    T: ModelTrait,
    <T as ModelTrait>::Entity: Related<R>,
    R: EntityTrait,
{
    async fn find_related_one(&mut self, related: R) -> Result<R::Model, DbErr>;
}

#[async_trait]
impl<T, R> GenericCrudRepositoryWithRelation<T, R> for T
where
    T: ModelTrait,
    <T as ModelTrait>::Entity: Related<R>,
    R: EntityTrait,
{
    async fn find_related_one(&mut self, related: R) -> Result<R::Model, DbErr> {
        let conn = get_database_connection().await;
        let result = self.find_related(related).one(conn).await?;

        match result {
            Some(model) => Ok(model),
            None => Err(DbErr::Custom("cant find related".into())),
        }
    }
}

#[async_trait]
pub trait GenericCrudRepository<T>
where
    T: EntityTrait,
{
    type PrimaryKeyType: Send;

    async fn get_all(&mut self) -> Result<Vec<T::Model>, DbErr>;

    async fn get_by_id(&mut self, id: Self::PrimaryKeyType) -> Result<Option<T::Model>, DbErr>;

    async fn update_by_model(&mut self, model: T::ActiveModel) -> Result<T::Model, DbErr>;

    async fn create(&mut self, model: T::ActiveModel) -> Result<T::Model, DbErr>;

    async fn delete_by_model_id(&mut self, id: Self::PrimaryKeyType) -> Result<u64, DbErr>;
}

#[async_trait]
impl<T> GenericCrudRepository<T> for T
where
    T: EntityTrait,
    T::ActiveModel: Send,
    T::Model: IntoActiveModel<T::ActiveModel>,
{
    type PrimaryKeyType = <T::PrimaryKey as PrimaryKeyTrait>::ValueType;

    async fn get_all(&mut self) -> Result<Vec<T::Model>, DbErr> {
        let conn = get_database_connection().await;
        let all_data = T::find().all(conn).await?;
        Ok(all_data)
    }

    async fn get_by_id(&mut self, id: Self::PrimaryKeyType) -> Result<Option<T::Model>, DbErr> {
        let conn = get_database_connection().await;
        T::find_by_id(id).one(conn).await
    }

    async fn update_by_model(&mut self, model: T::ActiveModel) -> Result<T::Model, DbErr> {
        let conn = get_database_connection().await;
        let updated_model: T::Model = model.update(conn).await?;
        Ok(updated_model)
    }

    async fn create(&mut self, model: T::ActiveModel) -> Result<T::Model, DbErr> {
        let conn = get_database_connection().await;
        let inserted_model: T::Model = model.insert(conn).await?;
        Ok(inserted_model)
    }

    async fn delete_by_model_id(&mut self, id: Self::PrimaryKeyType) -> Result<u64, DbErr> {
        let conn = get_database_connection().await;
        let deleted_result = T::delete_by_id(id).exec(conn).await.unwrap();
        Ok(deleted_result.rows_affected)
    }
}
