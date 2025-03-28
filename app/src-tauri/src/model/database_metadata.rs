use ams_macro::{GenerateDieselTable, GenerateTableEnum};
use chrono::{NaiveDate, Utc};
use diesel::{prelude::*, Selectable};
use sea_query::{Alias, ColumnDef, Iden, SeaRc, Table, TableRef};

use crate::{
    helper::sql_connection_helper::create_connection, migration::migration_trait::Migrationable,
};

#[derive(Insertable)]
#[diesel(table_name = metadata_table)]
pub struct MetadataNoId {
    pub version: i64,
    pub description: String,
}

#[derive(
    GenerateTableEnum, GenerateDieselTable, Queryable, Identifiable, Selectable, Debug, PartialEq,
)]
#[diesel(table_name = metadata_table)]
pub struct Metadata {
    pub id: i32,
    pub version: i64,
    pub description: String,
}

impl Migrationable for Metadata {
    fn get_up_migration(builder: impl sea_query::SchemaBuilder) -> String {
        Table::create()
            .table(MetadataTable::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(MetadataTable::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new(MetadataTable::Version).integer())
            .col(ColumnDef::new(MetadataTable::Description).text())
            .build(builder)
    }

    fn get_down_migration(builder: impl sea_query::SchemaBuilder) -> String {
        Table::drop()
            .table(MetadataTable::Table)
            .if_exists()
            .build(builder)
    }
}

impl Metadata {
    pub fn get_latest_version() -> i32 {
        let conn = &mut create_connection();

        let latest_db_metadata = metadata_table::table
            .order(metadata_table::version.desc())
            .first::<Metadata>(conn);

        match latest_db_metadata {
            Ok(db_metatdata) => db_metatdata.version as i32,
            Err(_) => 0,
        }
    }

    pub fn add_migration_stamp() {
        let conn = &mut create_connection();

        let latest_version = Self::get_latest_version();
        let new_db_metadata = MetadataNoId {
            version: (latest_version + 1) as i64,
            description: Utc::now().naive_utc().to_string(),
        };

        let _ = diesel::insert_into(metadata_table::table)
            .values(new_db_metadata)
            .execute(conn)
            .expect("Cant Insert to database metadata!!!");
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_get_latest_db_version() {
        let latest_version = Metadata::get_latest_version();

        println!("latest database version {latest_version}");
    }

    #[test]
    fn test_add_migration() {
        Metadata::add_migration_stamp();
    }
}
