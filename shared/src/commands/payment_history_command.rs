use crate::repositories::{
    abstract_repository_trait::AbstractRepository, database_connection::get_database_connection,
};
use ams_entity::{payment_history_table, prelude::*, taking_record_table};
use chrono::{Datelike, Days, Months, NaiveDate, NaiveDateTime};
use sea_orm::{EntityTrait, QueryFilter, entity::*, prelude::Expr};

pub trait PaymentHistoryCommandTrait {
    async fn get_payment_record_by_user_id(
        &mut self,
        user_id: i32,
    ) -> Vec<payment_history_table::Model>;

    async fn get_month_summary(&mut self, date: NaiveDateTime)
    -> Vec<payment_history_table::Model>;

    async fn get_payment_record_by_user_id_and_month(
        &mut self,
        user_id: i32,
        date: NaiveDateTime,
    ) -> Vec<payment_history_table::Model>;

    async fn get_month_summary_by_user_id(
        &mut self,
        user_id: i32,
        date: NaiveDateTime,
    ) -> Vec<payment_history_table::Model>;

    async fn update_payment_record(
        &mut self,
        record: payment_history_table::Model,
    ) -> payment_history_table::Model;

    async fn update_bulk_payment_record(
        &mut self,
        records: Vec<payment_history_table::Model>,
        paid: bool,
    ) -> u64;
}

pub struct PaymentHistoryCommad {
    payment_history_table: PaymentHistoryTable,
}

impl Default for PaymentHistoryCommad {
    fn default() -> Self {
        Self {
            payment_history_table: PaymentHistoryTable,
        }
    }
}

impl PaymentHistoryCommandTrait for PaymentHistoryCommad {
    async fn get_payment_record_by_user_id(
        &mut self,
        user_id: i32,
    ) -> Vec<payment_history_table::Model> {
        let conn = get_database_connection().await;

        PaymentHistoryTable::find()
            .filter(payment_history_table::Column::UserId.eq(user_id))
            .all(conn)
            .await
            .unwrap()
    }

    async fn get_payment_record_by_user_id_and_month(
        &mut self,
        user_id: i32,
        date: NaiveDateTime,
    ) -> Vec<payment_history_table::Model> {
        let conn = get_database_connection().await;

        let start_month = date
            .date()
            .with_day(1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();

        let end_month = if date.month() == 12 {
            NaiveDate::from_ymd_opt(date.year() + 1, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
        } else {
            NaiveDate::from_ymd_opt(date.year(), date.month() + 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
        };

        PaymentHistoryTable::find()
            .filter(payment_history_table::Column::UserId.eq(user_id))
            .filter(payment_history_table::Column::Date.lt(end_month))
            .filter(payment_history_table::Column::Date.gte(start_month))
            .all(conn)
            .await
            .unwrap()
    }

    async fn get_month_summary(
        &mut self,
        date: NaiveDateTime,
    ) -> Vec<payment_history_table::Model> {
        let conn = get_database_connection().await;

        let start_date = date
            .date()
            .with_day(1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();

        let end_date = start_date
            .clone()
            .date()
            .checked_add_months(Months::new(1))
            .unwrap()
            .with_day(1)
            .unwrap();

        PaymentHistoryTable::find()
            .filter(payment_history_table::Column::Date.lt(end_date))
            .filter(payment_history_table::Column::Date.gte(start_date))
            .all(conn)
            .await
            .unwrap()
    }

    async fn get_month_summary_by_user_id(
        &mut self,
        user_id: i32,
        date: NaiveDateTime,
    ) -> Vec<payment_history_table::Model> {
        let conn = get_database_connection().await;

        let start_date = date
            .date()
            .with_day(1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();

        let end_date = start_date
            .clone()
            .date()
            .checked_add_months(Months::new(1))
            .unwrap()
            .with_day(1)
            .unwrap();

        PaymentHistoryTable::find()
            .filter(payment_history_table::Column::UserId.eq(user_id))
            .filter(payment_history_table::Column::Date.lt(end_date))
            .filter(payment_history_table::Column::Date.gte(start_date))
            .all(conn)
            .await
            .unwrap()
    }

    async fn update_payment_record(
        &mut self,
        record: payment_history_table::Model,
    ) -> payment_history_table::Model {
        // TODO: this step is not optimal and good i think,
        // need to do refactor
        let active_model = payment_history_table::ActiveModel {
            id: NotSet,
            user_id: NotSet,
            date: NotSet,
            bill_amount: Set(record.bill_amount),
            initial_money: Set(record.initial_money),
            end_money: Set(record.end_money),
            added_money: Set(record.added_money),
        };

        self.payment_history_table
            .update_by_model(active_model)
            .await
            .unwrap()
    }

    async fn update_bulk_payment_record(
        &mut self,
        records: Vec<payment_history_table::Model>,
        paid: bool,
    ) -> u64 {
        let conn = get_database_connection().await;

        let start_date = records.iter().map(|item| item.date).min().unwrap();
        let end_date = records.iter().map(|item| item.date).max().unwrap();

        let start_date = start_date.date().and_hms_opt(0, 0, 0).unwrap();
        let end_date = end_date
            .checked_add_days(Days::new(1))
            .unwrap()
            .date()
            .and_hms_opt(0, 0, 0)
            .unwrap();

        let result = PaymentHistoryTable::update_many()
            .col_expr(taking_record_table::Column::IsPaid, Expr::value(paid))
            .filter(taking_record_table::Column::TakenDate.gte(start_date))
            .filter(taking_record_table::Column::TakenDate.lt(end_date))
            .exec(conn)
            .await
            .unwrap();

        result.rows_affected
    }
}

#[cfg(test)]
mod test {
    use chrono::Local;

    use super::*;
    #[test]
    fn check_date_with_day1() {
        let start_date = Local::now()
            .naive_local()
            .date()
            .with_day(1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();

        print!("start date is: {start_date:?}");
    }
}
