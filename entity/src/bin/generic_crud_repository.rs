use ams_entity::balance::{self, Entity as BalanceEntity};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{NotSet, Set},
    ConnectOptions, Database, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel,
    PrimaryKeyTrait,
    prelude::async_trait::async_trait,
};

async fn get_db_conn() -> DatabaseConnection {
    let mut connect_option = ConnectOptions::new("sqlite://ams.sqlite?mode=rwc");

    connect_option
        .max_connections(1)
        .min_connections(3)
        .sqlx_logging(true);

    Database::connect(connect_option).await.unwrap()
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
        let conn = get_db_conn().await;
        let all_data = T::find().all(&conn).await?;
        Ok(all_data)
    }

    async fn get_by_id(&mut self, id: Self::PrimaryKeyType) -> Result<Option<T::Model>, DbErr> {
        let conn = get_db_conn().await;
        T::find_by_id(id).one(&conn).await
    }

    async fn update_by_model(&mut self, model: T::ActiveModel) -> Result<T::Model, DbErr> {
        let conn = get_db_conn().await;
        let updated_model: T::Model = model.update(&conn).await?;
        Ok(updated_model)
    }

    async fn create(&mut self, model: T::ActiveModel) -> Result<T::Model, DbErr> {
        let conn = get_db_conn().await;
        let inserted_model: T::Model = model.insert(&conn).await?;
        Ok(inserted_model)
    }

    async fn delete_by_model_id(&mut self, id: Self::PrimaryKeyType) -> Result<u64, DbErr> {
        let conn = get_db_conn().await;
        let deleted_result = T::delete_by_id(id).exec(&conn).await.unwrap();
        Ok(deleted_result.rows_affected)
    }
}

#[tokio::main]
async fn main() {
    let mut connect_option = ConnectOptions::new("sqlite://ams.sqlite?mode=rwc");

    connect_option
        .max_connections(1)
        .min_connections(3)
        .sqlx_logging(true);

    let balance_model = balance::ActiveModel {
        balance_id: NotSet,
        customer_id: Set(1),
        value: Set(1),
        date: NotSet,
        transaction_type: Set(0),
    };

    let _ = BalanceEntity.create(balance_model).await;

    let db = Database::connect(connect_option).await.unwrap();
    let _ = BalanceEntity::find_by_id(1).one(&db).await;

    println!("hello");
}
