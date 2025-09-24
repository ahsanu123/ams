use ams_entity::{payment_history_table, prelude::*, taking_record_table};
use chrono::NaiveDateTime;
use sea_orm::{entity::*, prelude::async_trait::async_trait, query::*};

use crate::repositories::get_sql_connection_trait::GetSqlConnectionTrait;

#[async_trait]
pub trait AdditionalPaymentHistoryTableMethodTrait {
    async fn get_payment_history_by_user_id(user_id: i32) -> Vec<payment_history_table::Model>;
    async fn update_bulk(from: NaiveDateTime, to: NaiveDateTime, status: bool) -> i32;
    async fn update_record(
        record: payment_history_table::Model,
    ) -> Vec<payment_history_table::Model>;
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
    async fn update_bulk(from: NaiveDateTime, to: NaiveDateTime, status: bool) -> i32 {
        let conn = PaymentHistoryTable::get_connection().await;

        // let updated_taking_record = TakingRecordTable::update_many()
        // .col_expr(taking_record_table::Column::TakenDate, expr)
        todo!()
    }
    async fn update_record(
        record: payment_history_table::Model,
    ) -> Vec<payment_history_table::Model> {
        todo!()
    }
}
