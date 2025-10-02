use crate::{
    models::monthly_payment_summary::MonthlyPaymentSummary,
    repositories::abstract_repository_trait::AbstractRepository,
};
use ams_entity::{payment_history_table, prelude::*, taking_record_table};
use chrono::{Days, NaiveDateTime};
use sea_orm::{
    entity::*,
    prelude::{Expr, async_trait::async_trait},
    query::*,
};

use crate::repositories::get_sql_connection_trait::GetSqlConnectionTrait;

#[async_trait]
pub trait AdditionalPaymentHistoryTableMethodTrait {
    async fn get_payment_history_by_user_id(user_id: i32) -> Vec<payment_history_table::Model>;
    async fn update_payment_bulk(from: NaiveDateTime, to: NaiveDateTime, status: bool) -> u64;
    async fn update_payment_record(record: payment_history_table::Model) -> i32;
    async fn get_monthly_summary_by_user_id(
        user_id: i32,
        date: NaiveDateTime,
    ) -> MonthlyPaymentSummary;
}

#[async_trait]
impl AdditionalPaymentHistoryTableMethodTrait for PaymentHistoryTable {
    async fn get_payment_history_by_user_id(user_id: i32) -> Vec<payment_history_table::Model> {
        let conn = PaymentHistoryTable::get_connection().await;

        let result = PaymentHistoryTable::find()
            .filter(payment_history_table::Column::UserId.eq(user_id))
            .all(conn)
            .await
            .unwrap();

        result
    }
    async fn update_payment_bulk(from: NaiveDateTime, to: NaiveDateTime, status: bool) -> u64 {
        let conn = PaymentHistoryTable::get_connection().await;

        let from = from.date().and_hms_opt(0, 0, 0).unwrap();
        let to = to
            .date()
            .checked_add_days(Days::new(1))
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();

        let updated_taking_record = TakingRecordTable::update_many()
            .col_expr(taking_record_table::Column::IsPaid, Expr::value(status))
            .filter(taking_record_table::Column::TakenDate.gte(from))
            .filter(taking_record_table::Column::TakenDate.lt(to))
            .exec(conn)
            .await
            .unwrap();

        updated_taking_record.rows_affected
    }
    async fn update_payment_record(record: payment_history_table::Model) -> i32 {
        let active_model: payment_history_table::ActiveModel = record.into();
        let result = PaymentHistoryTable::update_by_model(active_model)
            .await
            .unwrap();

        result.id
    }

    async fn get_monthly_summary_by_user_id(
        user_id: i32,
        date: NaiveDateTime,
    ) -> MonthlyPaymentSummary {
        let conn = TakingRecordTable::get_connection().await;

        // TODO:
        // change date to filter by month,
        // still dont know how to do that

        let all_price = PriceHistoryTable::get_all().await.unwrap();
        // let default_price = all_price
        //     .first()
        //     .expect("please insert default price")
        //     .price;

        let taking_this_month: u32 = TakingRecordTable::find()
            .filter(taking_record_table::Column::UserId.eq(user_id))
            .filter(taking_record_table::Column::TakenDate.eq(date))
            .all(conn)
            .await
            .unwrap()
            .len() as u32;

        let total_paid_this_month: u32 = TakingRecordTable::find()
            .filter(taking_record_table::Column::UserId.eq(user_id))
            .filter(taking_record_table::Column::TakenDate.eq(date))
            .filter(taking_record_table::Column::IsPaid.eq(true))
            .all(conn)
            .await
            .unwrap()
            .len() as u32;

        let customer_bill = TakingRecordTable::find()
            .filter(taking_record_table::Column::UserId.eq(user_id))
            .filter(taking_record_table::Column::TakenDate.eq(date))
            .filter(taking_record_table::Column::IsPaid.eq(false))
            .all(conn)
            .await
            .unwrap()
            .iter()
            .map(|record| {
                let price = all_price
                    .iter()
                    .find(|pr| pr.id as i64 == record.price_id)
                    .unwrap()
                    .price;

                price * record.amount
            })
            .sum();

        let user_money = UserTable::get_by_id(user_id).await.unwrap().unwrap().money;

        MonthlyPaymentSummary {
            userid: user_id,
            total_taking: taking_this_month,
            total_paid: total_paid_this_month,
            bill: customer_bill,
            user_money: user_money,
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn name() {
        todo!();
    }
}
