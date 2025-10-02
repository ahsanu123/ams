use ams_entity::prelude::*;
use ams_entity::taking_record_table;
use chrono::Datelike;
use chrono::Local;
use chrono::Months;
use chrono::NaiveDateTime;
use sea_orm::{EntityTrait, entity::*, query::*};

use crate::repositories::abstract_repository_trait::AbstractRepository;
use crate::repositories::get_sql_connection_trait::GetSqlConnectionTrait;
use crate::repositories::price_repositories::AdditionalPriceHistoryTableMethodTrait;

pub async fn add_new_taking_record(user_id: i32, amount: i32) -> i32 {
    let latest_price = PriceHistoryTable::get_latest_price().await;

    let user = UserTable::get_by_id(user_id).await.unwrap();

    if user.is_none() {
        return 0;
    }

    let user = user.unwrap();

    let date_now = Local::now().naive_local();

    let active_model = taking_record_table::ActiveModel {
        id: NotSet,
        user_id: Set(user_id as i64),
        price_id: Set(latest_price.id as i64),
        amount: Set(amount as i64),
        production_date: Set(date_now),
        taken_date: Set(date_now),
        description: Set(format!("{0} is taking {1} dregs", user.username, amount)),
        is_paid: Set(false),
    };

    let result = TakingRecordTable::create(active_model).await.unwrap();

    result.id
}

pub async fn get_taking_record_by_user_id(user_id: i32) -> Vec<taking_record_table::Model> {
    let conn = TakingRecordTable::get_connection().await;

    let records = TakingRecordTable::find()
        .filter(taking_record_table::Column::UserId.eq(user_id))
        .all(conn)
        .await
        .unwrap();

    records
}

pub async fn upsert_taking_record(record: taking_record_table::Model) -> i32 {
    let data_on_db = TakingRecordTable::get_by_id(record.id).await.unwrap();

    if data_on_db.is_some() {
        let active_model = taking_record_table::ActiveModel {
            id: NotSet,
            user_id: NotSet,
            price_id: Set(record.price_id),
            amount: Set(record.amount),
            production_date: NotSet,
            taken_date: NotSet,
            description: NotSet,
            is_paid: Set(record.is_paid),
        };

        let update_result = TakingRecordTable::update_by_model(active_model)
            .await
            .unwrap();

        return update_result.id;
    } else {
        let active_model: taking_record_table::ActiveModel = data_on_db.unwrap().into();
        let create_result = TakingRecordTable::create(active_model).await.unwrap();

        return create_result.id;
    }
}

pub async fn get_taking_record_by_user_id_and_month(
    user_id: i32,
    date: NaiveDateTime,
) -> Vec<taking_record_table::Model> {
    let conn = TakingRecordTable::get_connection().await;

    let start_date = date
        .date()
        .with_day(1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();

    let end_date = start_date
        .clone()
        .checked_add_months(Months::new(1))
        .unwrap();

    let records = TakingRecordTable::find()
        .filter(taking_record_table::Column::TakenDate.gte(start_date))
        .filter(taking_record_table::Column::TakenDate.lt(end_date))
        .filter(taking_record_table::Column::UserId.eq(user_id))
        .all(conn)
        .await
        .unwrap();

    records
}

pub async fn get_taking_record_by_month(date: NaiveDateTime) -> Vec<taking_record_table::Model> {
    let conn = TakingRecordTable::get_connection().await;

    let start_date = date
        .date()
        .with_day(1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();

    let end_date = start_date
        .clone()
        .checked_add_months(Months::new(1))
        .unwrap();

    let records = TakingRecordTable::find()
        .filter(taking_record_table::Column::TakenDate.gte(start_date))
        .filter(taking_record_table::Column::TakenDate.lt(end_date))
        .all(conn)
        .await
        .unwrap();

    records
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn insert_new_taking_record() {
        let result = add_new_taking_record(3, 2).await;
        println!("result: {result:#?}");
    }

    #[tokio::test]
    async fn command_get_taking_record_by_user_id_and_month() {
        let date = Local::now()
            .naive_local()
            .date()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let result = get_taking_record_by_user_id_and_month(1, date).await;

        println!("result: {result:#?}");
    }
}
