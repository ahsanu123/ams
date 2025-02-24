use sea_query::{SchemaBuilder, TableCreateStatement};

pub trait MigrationAble {
    fn get_up_migration(builder: impl SchemaBuilder) -> String;
    fn get_down_migration(builder: impl SchemaBuilder) -> String;
}
