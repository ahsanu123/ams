use std::fs;
use std::path::Path;

use migration::sea_orm::Database;
use migration::{MigratorTrait, MigratorWithSeed};

#[async_std::main]
async fn main() {
    let database_path = "./../app/src-tauri/ams.sqlite";

    if let Some(parent) = Path::new(database_path).parent() {
        fs::create_dir_all(parent).unwrap()
    }

    let sqlite_file = format!("sqlite://{database_path}?mode=rwc");
    let connection = Database::connect(sqlite_file).await.unwrap();

    // TODO: think on how to migrate old data into new table
    // Migrator::down(&connection, None).await.unwrap();
    MigratorWithSeed::up(&connection, None).await.unwrap();
}
