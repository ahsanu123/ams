use super::user::{user_table, UserTable};
use crate::migration::migration_trait::Migrationable;
use ams_macro::{GenerateDieselTable, GenerateTableEnum};
use diesel::prelude::*;
use sea_query::{ColumnDef, ForeignKey, ForeignKeyAction, Iden, SchemaBuilder, Table};

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = hash_table)]
pub struct HashNoId {
    pub user_id: i32,
    pub hash: String,
}

#[derive(
    GenerateTableEnum,
    GenerateDieselTable,
    Queryable,
    Identifiable,
    Selectable,
    Debug,
    PartialEq,
    Insertable,
)]
#[diesel(table_name = hash_table)]
#[diesel(belongs_to(User))]
pub struct Hash {
    pub id: i32,
    pub user_id: i32,
    pub hash: String,
}

diesel::joinable!(hash_table -> user_table(user_id));
diesel::allow_tables_to_appear_in_same_query!(user_table, hash_table);

impl Migrationable for Hash {
    fn get_up_migration(builder: impl SchemaBuilder) -> String {
        Table::create()
            .table(HashTable::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(HashTable::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new(HashTable::Hash).text())
            .col(ColumnDef::new(HashTable::UserId).integer())
            .foreign_key(
                ForeignKey::create()
                    .name("FK_HashToUser")
                    .from(HashTable::Table, HashTable::UserId)
                    .to(UserTable::Table, UserTable::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade),
            )
            .build(builder)
    }

    fn get_down_migration(builder: impl SchemaBuilder) -> String {
        Table::drop()
            .table(HashTable::Table)
            .if_exists()
            .build(builder)
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn check_hash_table() {
        todo!();
    }
}
