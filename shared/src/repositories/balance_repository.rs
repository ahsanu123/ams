use crate::{
    models::balance::Balance,
    repositories::base_repository_trait::{BaseRepository, BaseRepositoryErr},
};

pub enum BalanceRepositoryErr {
    FailToGetByCustomerId,
    FailToUpdateMany,
}

pub struct BalanceRepository {}

impl BalanceRepository {
    pub fn get_by_customer_id(&mut self, id: i64) -> Result<Vec<Balance>, BalanceRepositoryErr> {
        todo!()
    }

    pub fn update_many(&mut self, models: Vec<Balance>) -> Result<i64, BalanceRepositoryErr> {
        todo!()
    }
}

impl BaseRepository<Balance> for BalanceRepository {
    async fn create(&mut self, model: Balance) -> Result<i64, BaseRepositoryErr> {
        todo!()
    }

    async fn read(&mut self, id: i64) -> Result<Option<Balance>, BaseRepositoryErr> {
        todo!()
    }

    async fn update(&mut self, model: Balance) -> Result<Balance, BaseRepositoryErr> {
        todo!()
    }

    async fn delete(&mut self, id: i64) -> Result<u64, BaseRepositoryErr> {
        todo!()
    }
}
// use crate::{
//     models::monthly_payment_summary::MonthlyPaymentSummary,
//     repositories::{
//         abstract_repository_trait::AbstractRepository, database_connection::get_database_connection,
//     },
// };
// use ams_entity::{payment_history_table, prelude::*, taking_record_table};
// use chrono::{Days, NaiveDateTime};
// use sea_orm::{entity::*, prelude::Expr, query::*};
//
// pub trait AdditionalPaymentHistoryRepoTrait {
//     async fn get_payment_history_by_user_id(
//         &mut self,
//         user_id: i32,
//     ) -> Vec<payment_history_table::Model>;
//
//     async fn update_payment_bulk(
//         &mut self,
//         from: NaiveDateTime,
//         to: NaiveDateTime,
//         status: bool,
//     ) -> u64;
//
//     async fn update_payment_record(&mut self, record: payment_history_table::Model) -> i32;
//
//     async fn get_monthly_summary_by_user_id(
//         &mut self,
//         user_id: i32,
//         date: NaiveDateTime,
//     ) -> MonthlyPaymentSummary;
// }
//
// pub struct PaymentHistoryRepository {
//     payment_history_table: PaymentHistoryTable,
//     user_table: UserTable,
//     price_history_table: PriceHistoryTable,
// }
//
// impl PaymentHistoryRepository {
//     pub fn new(
//         payment_history_table: PaymentHistoryTable,
//         user_table: UserTable,
//         price_history_table: PriceHistoryTable,
//     ) -> Self {
//         Self {
//             payment_history_table,
//             user_table,
//             price_history_table,
//         }
//     }
// }
//
// impl AdditionalPaymentHistoryRepoTrait for PaymentHistoryRepository {
//     async fn get_payment_history_by_user_id(
//         &mut self,
//         user_id: i32,
//     ) -> Vec<payment_history_table::Model> {
//         let conn = get_database_connection().await;
//
//         let result = PaymentHistoryTable::find()
//             .filter(payment_history_table::Column::UserId.eq(user_id))
//             .all(conn)
//             .await
//             .unwrap();
//
//         result
//     }
//     async fn update_payment_bulk(
//         &mut self,
//         from: NaiveDateTime,
//         to: NaiveDateTime,
//         status: bool,
//     ) -> u64 {
//         let conn = get_database_connection().await;
//
//         let from = from.date().and_hms_opt(0, 0, 0).unwrap();
//         let to = to
//             .date()
//             .checked_add_days(Days::new(1))
//             .unwrap()
//             .and_hms_opt(0, 0, 0)
//             .unwrap();
//
//         let updated_taking_record = TakingRecordTable::update_many()
//             .col_expr(taking_record_table::Column::IsPaid, Expr::value(status))
//             .filter(taking_record_table::Column::TakenDate.gte(from))
//             .filter(taking_record_table::Column::TakenDate.lt(to))
//             .exec(conn)
//             .await
//             .unwrap();
//
//         updated_taking_record.rows_affected
//     }
//     async fn update_payment_record(&mut self, record: payment_history_table::Model) -> i32 {
//         let active_model: payment_history_table::ActiveModel = record.into();
//
//         let result = self
//             .payment_history_table
//             .update_by_model(active_model)
//             .await
//             .unwrap();
//
//         result.id
//     }
//
//     async fn get_monthly_summary_by_user_id(
//         &mut self,
//         user_id: i32,
//         date: NaiveDateTime,
//     ) -> MonthlyPaymentSummary {
//         let conn = get_database_connection().await;
//
//         // TODO:
//         // change date to filter by month,
//         // still dont know how to do that
//
//         let all_price = self.price_history_table.get_all().await.unwrap();
//
//         // let default_price = all_price
//         //     .first()
//         //     .expect("please insert default price")
//         //     .price;
//
//         let taking_this_month: u32 = TakingRecordTable::find()
//             .filter(taking_record_table::Column::UserId.eq(user_id))
//             .filter(taking_record_table::Column::TakenDate.eq(date))
//             .all(conn)
//             .await
//             .unwrap()
//             .len() as u32;
//
//         let total_paid_this_month: u32 = TakingRecordTable::find()
//             .filter(taking_record_table::Column::UserId.eq(user_id))
//             .filter(taking_record_table::Column::TakenDate.eq(date))
//             .filter(taking_record_table::Column::IsPaid.eq(true))
//             .all(conn)
//             .await
//             .unwrap()
//             .len() as u32;
//
//         let customer_bill = TakingRecordTable::find()
//             .filter(taking_record_table::Column::UserId.eq(user_id))
//             .filter(taking_record_table::Column::TakenDate.eq(date))
//             .filter(taking_record_table::Column::IsPaid.eq(false))
//             .all(conn)
//             .await
//             .unwrap()
//             .iter()
//             .map(|record| {
//                 let price = all_price
//                     .iter()
//                     .find(|pr| pr.id as i64 == record.price_id)
//                     .unwrap()
//                     .price;
//
//                 price * record.amount
//             })
//             .sum();
//
//         let user_money = self
//             .user_table
//             .get_by_id(user_id)
//             .await
//             .unwrap()
//             .unwrap()
//             .money;
//
//         MonthlyPaymentSummary {
//             userid: user_id,
//             total_taking: taking_this_month,
//             total_paid: total_paid_this_month,
//             bill: customer_bill,
//             user_money,
//         }
//     }
// }
