use ams_entity::{prelude::*, production_record_table, taking_record_table};
use chrono::{Datelike, Month, Months, NaiveDateTime};
use sea_orm::{entity::*, prelude::async_trait::async_trait, query::*};

use crate::repositories::{
    abstract_repository_trait::AbstractRepository, get_sql_connection_trait::GetSqlConnectionTrait,
};

#[async_trait]
pub trait AdditionalProductionRecordTableMethodTrait {
    async fn get_production_record_by_month(
        date: NaiveDateTime,
    ) -> Vec<production_record_table::Model>;
}

#[async_trait]
impl AdditionalProductionRecordTableMethodTrait for ProductionRecordTable {
    async fn get_production_record_by_month(
        date: NaiveDateTime,
    ) -> Vec<production_record_table::Model> {
        let conn = ProductionRecordTable::get_connection().await;

        let start_month = date
            .with_day(1)
            .unwrap()
            .date()
            .and_hms_opt(0, 0, 0)
            .unwrap();

        let end_month = date
            .checked_add_months(Months::new(1))
            .unwrap()
            .with_day(1)
            .unwrap()
            .date()
            .and_hms_opt(0, 0, 0)
            .unwrap();

        let records = ProductionRecordTable::find()
            .filter(production_record_table::Column::Date.gte(start_month))
            .filter(production_record_table::Column::Date.lt(end_month))
            .all(conn)
            .await
            .unwrap();

        records
    }
}
