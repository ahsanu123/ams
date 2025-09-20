use migration::sea_orm;
use migration::{Migrator, MigratorTrait};

#[async_std::main]
async fn main() {
    let path = "sqlite://./ams.sqlite?mode=rwc";
    let connection = sea_orm::Database::connect(path).await.unwrap();

    Migrator::up(&connection, None).await.unwrap();
}
