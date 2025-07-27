use ams_migration::{sea_orm::Database, MigratorTrait};

#[async_std::main]
async fn main() {
    let db = Database::connect("sqlite://db.sqlite?mode=rwc")
        .await
        .unwrap();

    let _ = ams_migration::Migrator::up(&db, None).await;
}
