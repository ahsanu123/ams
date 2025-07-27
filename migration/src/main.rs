use migration::sea_orm::Database;
use sea_orm_migration::prelude::*;

#[async_std::main]
async fn main() {
    let db = Database::connect("sqlite://db.sqlite?mode=rwc")
        .await
        .unwrap();

    let _ = migration::Migrator::up(&db, None).await;
}
