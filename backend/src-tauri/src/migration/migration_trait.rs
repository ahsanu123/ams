use sea_query::TableCreateStatement;

pub trait MigrationAble {
    fn get_up_migration() -> TableCreateStatement;
    fn get_down_migration() -> bool;
}
