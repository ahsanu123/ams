use sea_query::SchemaBuilder;

pub trait Migrationable {
    fn get_up_migration(builder: impl SchemaBuilder) -> String;
    fn get_down_migration(builder: impl SchemaBuilder) -> String;
}
