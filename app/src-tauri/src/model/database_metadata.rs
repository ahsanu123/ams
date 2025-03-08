use custom_macro::{GenerateDieselTable, GenerateTableEnum};
use diesel::{prelude::Insertable, Selectable};
use sea_query::{ColumnDef, Iden, Table};

use crate::migration::migration_trait::Migrationable;

// GenerateTableEnum
// implement Migrationable
// GenerateDieselTable
// create struct no id
#[derive(Insertable)]
#[diesel(table_name = databasemetadata_table)]
pub struct DatabaseMetadataNoId {
    pub version: i64,
    pub description: String,
}

#[derive(GenerateTableEnum, GenerateDieselTable, Selectable)]
#[diesel(table_name = databasemetadata_table)]
pub struct DatabaseMetadata {
    pub id: i32,
    pub version: i64,
    pub description: String,
}

impl Migrationable for DatabaseMetadata {
    fn get_up_migration(builder: impl sea_query::SchemaBuilder) -> String {
        Table::create()
            .table(DatabaseMetadataTable::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(DatabaseMetadataTable::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new(DatabaseMetadataTable::Version).integer())
            .col(ColumnDef::new(DatabaseMetadataTable::Description).text())
            .build(builder)
    }

    fn get_down_migration(builder: impl sea_query::SchemaBuilder) -> String {
        Table::drop()
            .table(DatabaseMetadataTable::Table)
            .if_exists()
            .build(builder)
    }
}
impl DatabaseMetadata {
    pub fn add_migration(db_metadata: DatabaseMetadata) {
        todo!()
    }
    pub fn get_latest_version() {
        todo!()
    }
}
