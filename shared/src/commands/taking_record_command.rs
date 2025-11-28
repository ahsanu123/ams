use itertools::Itertools;

use crate::repositories::{
    abstract_repository_trait::AbstractRepository, get_sql_connection_trait::GetSqlConnectionTrait,
    price_repositories::AdditionalPriceHistoryTableMethodTrait,
};
use ams_entity::{prelude::*, taking_record_table};
use chrono::{Datelike, Days, Local, Months, NaiveDateTime};
use sea_orm::{EntityTrait, entity::*, prelude::async_trait, query::*};

#[async_trait::async_trait]
pub trait TakingRecordCommandTrait {
    async fn add_new_taking_record(user_id: i32, amount: i32) -> i32;

    async fn add_new_taking_record_by_date(user_id: i32, amount: i32, date: NaiveDateTime) -> i32;

    async fn get_taking_record_by_user_id(user_id: i32) -> Vec<taking_record_table::Model>;

    async fn upsert_taking_record_by_date(user_id: i32, amount: i32, date: NaiveDateTime) -> i32;

    async fn upsert_taking_record(record: taking_record_table::Model) -> i32;

    async fn get_taking_record_by_month(date: NaiveDateTime) -> Vec<taking_record_table::Model>;

    async fn get_taking_record_by_day(date: NaiveDateTime) -> Vec<taking_record_table::Model>;

    async fn delete_taking_record(record_id: i32) -> u64;

    async fn get_taking_record_by_user_id_and_year(
        user_id: i32,
        date: NaiveDateTime,
    ) -> Vec<taking_record_table::Model>;

    async fn get_taking_record_by_user_id_and_month(
        user_id: i32,
        date: NaiveDateTime,
    ) -> Vec<taking_record_table::Model>;

    async fn get_taking_record_by_user_id_and_month_range(
        user_id: i32,
        from: NaiveDateTime,
        to: NaiveDateTime,
    ) -> Vec<taking_record_table::Model>;
}

pub struct TakingRecordCommand;

#[async_trait::async_trait]
impl TakingRecordCommandTrait for TakingRecordCommand {
    async fn upsert_taking_record_by_date(user_id: i32, amount: i32, date: NaiveDateTime) -> i32 {
        let conn = TakingRecordTable::get_connection().await;

        let user = UserTable::get_by_id(user_id).await.unwrap();

        if user.is_none() {
            return 0;
        }

        let record_at_that_date = TakingRecordTable::find()
            .filter(taking_record_table::Column::UserId.eq(user_id))
            .filter(taking_record_table::Column::TakenDate.eq(date))
            .one(conn)
            .await
            .unwrap();

        if let Some(data) = record_at_that_date {
            let active_model = taking_record_table::ActiveModel {
                id: Set(data.id),
                user_id: NotSet,
                price_id: NotSet,
                amount: Set(amount as i64),
                production_date: NotSet,
                taken_date: NotSet,
                description: NotSet,
                is_paid: NotSet,
            };

            let res = TakingRecordTable::update_by_model(active_model)
                .await
                .unwrap();
            return res.id;
        } else {
            let res = TakingRecordCommand::add_new_taking_record(user_id, amount).await;
            return res;
        }
    }

    async fn add_new_taking_record_by_date(user_id: i32, amount: i32, date: NaiveDateTime) -> i32 {
        let latest_price = PriceHistoryTable::get_latest_price().await;

        let user = UserTable::get_by_id(user_id).await.unwrap();

        if user.is_none() {
            return 0;
        }

        let user = user.unwrap();

        let active_model = taking_record_table::ActiveModel {
            id: NotSet,
            user_id: Set(user_id as i64),
            price_id: Set(latest_price.id as i64),
            amount: Set(amount as i64),
            production_date: Set(date),
            taken_date: Set(date),
            description: Set(format!("{0} is taking {1} dregs", user.username, amount)),
            is_paid: Set(false),
        };

        let result = TakingRecordTable::create(active_model).await.unwrap();

        result.id
    }

    async fn add_new_taking_record(user_id: i32, amount: i32) -> i32 {
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

    async fn get_taking_record_by_user_id(user_id: i32) -> Vec<taking_record_table::Model> {
        let conn = TakingRecordTable::get_connection().await;

        let records = TakingRecordTable::find()
            .filter(taking_record_table::Column::UserId.eq(user_id))
            .all(conn)
            .await
            .unwrap();

        records
    }

    async fn upsert_taking_record(record: taking_record_table::Model) -> i32 {
        let data_on_db = TakingRecordTable::get_by_id(record.id).await.unwrap();

        if data_on_db.is_some() {
            let active_model = taking_record_table::ActiveModel {
                id: Set(record.id),
                user_id: Set(record.user_id),
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
            #[allow(clippy::panicking_unwrap)]
            let active_model: taking_record_table::ActiveModel = data_on_db.unwrap().into();
            let create_result = TakingRecordTable::create(active_model).await.unwrap();

            return create_result.id;
        }
    }

    async fn get_taking_record_by_user_id_and_month(
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

        let summed_records = records
            .iter()
            .into_group_map_by(|item| item.taken_date.date())
            .into_iter()
            .map(|(_, records)| {
                let total_taking = records.iter().map(|item| item.amount).sum::<i64>();

                let mut first_data = (*records.first().unwrap()).clone();
                first_data.amount = total_taking;

                first_data
            })
            .collect::<Vec<taking_record_table::Model>>();

        summed_records
    }

    async fn delete_taking_record(record_id: i32) -> u64 {
        let res = TakingRecordTable::delete_by_model_id(record_id)
            .await
            .unwrap();

        res
    }

    async fn get_taking_record_by_day(date: NaiveDateTime) -> Vec<taking_record_table::Model> {
        let conn = TakingRecordTable::get_connection().await;

        let start_date = date.date().and_hms_opt(0, 0, 0).unwrap();

        let end_date = start_date.clone().checked_add_days(Days::new(1)).unwrap();

        let records = TakingRecordTable::find()
            .filter(taking_record_table::Column::TakenDate.gte(start_date))
            .filter(taking_record_table::Column::TakenDate.lt(end_date))
            .all(conn)
            .await
            .unwrap();

        records
    }

    async fn get_taking_record_by_user_id_and_year(
        user_id: i32,
        date: NaiveDateTime,
    ) -> Vec<taking_record_table::Model> {
        let conn = TakingRecordTable::get_connection().await;

        let start_year = date
            .date()
            .with_month(1)
            .unwrap()
            .with_day(1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();

        let end_year = start_year
            .clone()
            .with_year(date.year() + 1)
            .unwrap()
            .checked_add_months(Months::new(1))
            .unwrap();

        let records = TakingRecordTable::find()
            .filter(taking_record_table::Column::TakenDate.gte(start_year))
            .filter(taking_record_table::Column::TakenDate.lt(end_year))
            .filter(taking_record_table::Column::UserId.eq(user_id))
            .order_by(taking_record_table::Column::TakenDate, Order::Asc)
            .all(conn)
            .await
            .unwrap();

        let summed_records_in_year = records
            .iter()
            .into_group_map_by(|item| item.taken_date.date())
            .into_iter()
            .map(|(_, records)| {
                let total_taking = records.iter().map(|item| item.amount).sum::<i64>();

                let mut first_data = (*records.first().unwrap()).clone();
                first_data.amount = total_taking;

                first_data
            })
            .collect::<Vec<taking_record_table::Model>>();

        summed_records_in_year
    }

    async fn get_taking_record_by_month(date: NaiveDateTime) -> Vec<taking_record_table::Model> {
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
            .iter()
            .into_group_map_by(|item| item.taken_date.date())
            .into_iter()
            .map(|(_, records)| {
                let total_taking = records.iter().map(|item| item.amount).sum::<i64>();

                let mut first_data = (*records.first().unwrap()).clone();
                first_data.amount = total_taking;

                first_data
            })
            .collect::<Vec<taking_record_table::Model>>()
    }

    async fn get_taking_record_by_user_id_and_month_range(
        user_id: i32,
        from: NaiveDateTime,
        to: NaiveDateTime,
    ) -> Vec<taking_record_table::Model> {
        let conn = TakingRecordTable::get_connection().await;

        let start_date = from
            .date()
            .with_day(1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();

        let end_date = to
            .date()
            .with_day(1)
            .unwrap()
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
            .iter()
            .into_group_map_by(|item| item.taken_date.date())
            .into_iter()
            .map(|(_, records)| {
                let total_taking = records.iter().map(|item| item.amount).sum::<i64>();

                let mut first_data = (*records.first().unwrap()).clone();
                first_data.amount = total_taking;

                first_data
            })
            .collect::<Vec<taking_record_table::Model>>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn insert_new_taking_record() {
        let result = TakingRecordCommand::add_new_taking_record(3, 2).await;
        println!("result: {result:#?}");
    }

    #[tokio::test]
    async fn command_get_taking_record_by_user_id_and_month() {
        let date = Local::now()
            .naive_local()
            .date()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let result = TakingRecordCommand::get_taking_record_by_user_id_and_month(1, date).await;

        println!("result: {result:#?}");
    }
}
