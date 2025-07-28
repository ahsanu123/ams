use ams_migration::{sea_orm::Database, MigratorTrait};

#[async_std::main]
async fn main() {
    let db = Database::connect("sqlite://db.sqlite?mode=rwc")
        .await
        .unwrap();

    let _ = ams_migration::Migrator::up(&db, None).await;
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn main_migrate_up() {
        let db = Database::connect("sqlite://db.sqlite?mode=rwc")
            .await
            .unwrap();

        let _ = ams_migration::Migrator::fresh(&db).await;
    }

    #[tokio::test]
    async fn main_migrate_down() {
        let db = Database::connect("sqlite://db.sqlite?mode=rwc")
            .await
            .unwrap();

        let _ = ams_migration::Migrator::reset(&db).await;
    }
}
