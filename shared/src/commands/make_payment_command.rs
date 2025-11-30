use crate::models::make_payment_page_model::{
    DetailInformation, MakePaymentPageModel, TakingRecordWithPrice,
};
use crate::repositories::abstract_repository_trait::AbstractRepository;
use crate::repositories::get_sql_connection_trait::GetSqlConnectionTrait;
use crate::repositories::user_repository::AdditionalUserTableMethodTrait;
use crate::utilities::format_as_idr::format_as_idr;
use ams_entity::{
    money_history_table, payment_history_table, prelude::*, taking_record_table, user_table,
};
use chrono::{Datelike, Local, Months, NaiveDateTime};
use sea_orm::prelude::Expr;
use sea_orm::{
    ActiveValue::{NotSet, Set},
    EntityTrait, QueryFilter,
    entity::*,
    prelude::async_trait,
};
use std::fmt::Error;

#[async_trait::async_trait]
pub trait MakePaymentCommandTrait {
    async fn get_page_model(
        user_id: i32,
        date: NaiveDateTime,
    ) -> Result<MakePaymentPageModel, Error>;

    async fn make_payment(user_id: i32, date: NaiveDateTime)
    -> Result<MakePaymentPageModel, Error>;
}

pub struct MakePaymentCommand;

#[async_trait::async_trait]
impl MakePaymentCommandTrait for MakePaymentCommand {
    async fn get_page_model(
        user_id: i32,
        date: NaiveDateTime,
    ) -> Result<MakePaymentPageModel, Error> {
        let conn = TakingRecordTable::get_connection().await;

        let start_month = date
            .date()
            .with_day(1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();

        let end_month = start_month.checked_add_months(Months::new(1)).unwrap();

        let taking_record_with_price = TakingRecordTable::find()
            .filter(taking_record_table::Column::UserId.eq(user_id))
            .filter(taking_record_table::Column::TakenDate.gte(start_month))
            .filter(taking_record_table::Column::TakenDate.lt(end_month))
            .find_with_related(PriceHistoryTable)
            .all(conn)
            .await
            .unwrap()
            .iter()
            .map(|item| TakingRecordWithPrice {
                taking_record: item.0.clone(),
                price: item.1.first().unwrap().clone(),
            })
            .collect::<Vec<TakingRecordWithPrice>>();

        let total_bill = taking_record_with_price
            .iter()
            .map(|record| {
                let bill = record.taking_record.amount * record.price.price;
                bill
            })
            .sum::<i64>();

        let paid_bill = taking_record_with_price
            .iter()
            .filter(|pr| pr.taking_record.is_paid)
            .map(|record| {
                let bill = record.taking_record.amount * record.price.price;
                bill
            })
            .sum::<i64>();

        let unpaid_bill = taking_record_with_price
            .iter()
            .filter(|pr| !pr.taking_record.is_paid)
            .map(|record| {
                let bill = record.taking_record.amount * record.price.price;
                bill
            })
            .sum::<i64>();

        let total_amount = taking_record_with_price
            .iter()
            .map(|record| record.taking_record.amount)
            .sum::<i64>();

        let paid_amount = taking_record_with_price
            .iter()
            .filter(|pr| pr.taking_record.is_paid)
            .map(|record| record.taking_record.amount)
            .sum::<i64>();

        let unpaid_amount = taking_record_with_price
            .iter()
            .filter(|pr| !pr.taking_record.is_paid)
            .map(|record| record.taking_record.amount)
            .sum::<i64>();

        let active_customer = UserTable::get_all_active_user().await;

        Ok(MakePaymentPageModel {
            taking_records: taking_record_with_price,
            customers: active_customer,
            detail_information: DetailInformation {
                total_bill,
                total_amount,
                paid_bill,
                paid_amount,
                unpaid_bill,
                unpaid_amount,
            },
        })
    }

    async fn make_payment(
        user_id: i32,
        date: NaiveDateTime,
    ) -> Result<MakePaymentPageModel, Error> {
        let conn = TakingRecordTable::get_connection().await;

        let start_month = date
            .date()
            .with_day(1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();

        let end_month = start_month.checked_add_months(Months::new(1)).unwrap();

        let customer = UserTable::get_by_id(user_id).await.unwrap();

        if customer.is_none() {
            return Err(Error);
        }

        let customer = customer.unwrap();

        let taking_record_with_price = TakingRecordTable::find()
            .filter(taking_record_table::Column::IsPaid.eq(false))
            .filter(taking_record_table::Column::UserId.eq(user_id))
            .filter(taking_record_table::Column::TakenDate.gte(start_month))
            .filter(taking_record_table::Column::TakenDate.lt(end_month))
            .find_with_related(PriceHistoryTable)
            .all(conn)
            .await
            .unwrap()
            .iter()
            .map(|item| TakingRecordWithPrice {
                taking_record: item.0.clone(),
                price: item.1.first().unwrap().clone(),
            })
            .collect::<Vec<TakingRecordWithPrice>>();

        let _updated_taking_record = TakingRecordTable::update_many()
            .col_expr(taking_record_table::Column::IsPaid, Expr::value(true))
            .filter(taking_record_table::Column::UserId.eq(user_id))
            .filter(taking_record_table::Column::TakenDate.gte(start_month))
            .filter(taking_record_table::Column::TakenDate.lt(end_month))
            .filter(taking_record_table::Column::IsPaid.eq(false))
            .exec(conn)
            .await
            .unwrap();

        let total_bill = taking_record_with_price
            .iter()
            .map(|record| record.taking_record.amount * record.price.price)
            .sum::<i64>();

        // doesnt need to update anything
        if total_bill == 0 {
            return Self::get_page_model(user_id, date).await;
        }

        let final_money = customer.money - total_bill;

        let description = format!(
            "{0} is paying Rp.{1}, initial money Rp.{2}, final money Rp.{3}",
            customer.username,
            format_as_idr(total_bill),
            format_as_idr(customer.money),
            format_as_idr(final_money)
        );

        let _insert_payment_history =
            PaymentHistoryTable::create(payment_history_table::ActiveModel {
                id: NotSet,
                user_id: Set(user_id as i64),
                date: Set(Local::now().naive_local()),
                bill_amount: Set(total_bill),
                initial_money: Set(customer.money),
                end_money: Set(final_money),
                added_money: Set(0),
            })
            .await
            .unwrap();

        let _insert_customer_money_history =
            MoneyHistoryTable::create(money_history_table::ActiveModel {
                id: NotSet,
                user_id: Set(user_id as i64),
                date: Set(Local::now().naive_local()),
                money_amount: Set(final_money),
                description: Set(description.into()),
            })
            .await
            .unwrap();

        let mut customer_active_model: user_table::ActiveModel = customer.into();
        customer_active_model.money = Set(final_money);

        let _updated_customer = customer_active_model.update(conn).await.unwrap();

        Self::get_page_model(user_id, date).await
    }
}

#[cfg(test)]
mod make_payment_command_test {

    use super::*;

    #[tokio::test]
    async fn test_get_page_model() {
        let page_model = MakePaymentCommand::get_page_model(1, Local::now().naive_local())
            .await
            .expect("can't find requested data!!!");

        println!("{page_model:#?}");
    }

    #[tokio::test]
    async fn test_make_payment() {
        let page_model = MakePaymentCommand::make_payment(1, Local::now().naive_local())
            .await
            .expect("somethink went wrong");

        println!("{page_model:#?}");
    }
}
