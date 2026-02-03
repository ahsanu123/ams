use crate::{
    repositories::{
        abstract_repository_trait::AbstractRepository, database_connection::get_database_connection,
    },
    utilities::format_as_idr::format_as_idr,
};
use ams_entity::{
    money_history_table, payment_history_table, prelude::*, taking_record_table, user_table,
};
use chrono::{Days, Local, NaiveDateTime};
use sea_orm::{
    ActiveValue::{NotSet, Set},
    EntityTrait, Order, QueryFilter, QueryOrder,
    entity::*,
    prelude::Expr,
};
use std::fmt::Error;

trait CustomerMoneyCommandTrait {
    async fn add_money(&mut self, user_id: i64, amount: i64) -> Result<user_table::Model, Error>;
    async fn delete_user(&mut self, user_id: i32) -> Result<u64, Error>;
    async fn get_all_user_money_history(&mut self, user_id: i64)
    -> Vec<money_history_table::Model>;
    async fn make_payment(
        &mut self,
        user_id: i64,
        from: NaiveDateTime,
        to: NaiveDateTime,
    ) -> Option<user_table::Model>;
}

pub struct CustomerMoneyCommand {
    user_table: UserTable,
    money_history_table: MoneyHistoryTable,
    payment_history_table: PaymentHistoryTable,
}

impl CustomerMoneyCommandTrait for CustomerMoneyCommand {
    async fn add_money(&mut self, user_id: i64, amount: i64) -> Result<user_table::Model, Error> {
        let user = self.user_table.get_by_id(user_id as i32).await.unwrap();

        if user.is_none() {
            return Err(Error);
        }

        let user = user.unwrap();

        let mut active_model: user_table::ActiveModel = user.clone().into();
        active_model.money = Set(user.money + amount);

        // update user money
        let updated_user = self.user_table.update_by_model(active_model).await.unwrap();

        // insert history to money history
        let money_history = money_history_table::ActiveModel {
            id: NotSet,
            user_id: Set(user_id),
            date: Set(Local::now().naive_local()),
            money_amount: Set(amount),
            description: Set(format!(
                "Add Money Rp.{0}, final money amount Rp.{1}",
                format_as_idr(amount),
                format_as_idr(updated_user.money)
            )),
        };

        let _ = self
            .money_history_table
            .create(money_history)
            .await
            .unwrap();

        // user is have dept
        if user.money < 0 {
            let _insert_customer_money_history = self
                .money_history_table
                .create(money_history_table::ActiveModel {
                    id: NotSet,
                    user_id: Set(user_id),
                    date: Set(Local::now().naive_local()),
                    money_amount: Set(updated_user.money),
                    description: Set(format!(
                        "Paying Dept Rp.{0}, final money Rp.{1}",
                        format_as_idr(user.money.abs()),
                        format_as_idr(updated_user.money)
                    )),
                })
                .await
                .unwrap();

            let _insert_payment_history = self
                .payment_history_table
                .create(payment_history_table::ActiveModel {
                    id: NotSet,
                    user_id: Set(user_id),
                    date: Set(Local::now().naive_local()),
                    bill_amount: Set(user.money.abs()),
                    initial_money: Set(user.money),
                    end_money: Set(updated_user.money),
                    added_money: Set(amount),
                })
                .await
                .unwrap();
        }

        Ok(updated_user)
    }

    async fn get_all_user_money_history(
        &mut self,
        user_id: i64,
    ) -> Vec<money_history_table::Model> {
        let conn = get_database_connection().await;

        MoneyHistoryTable::find()
            .filter(money_history_table::Column::UserId.eq(user_id))
            .order_by(money_history_table::Column::Date, Order::Desc)
            .all(conn)
            .await
            .unwrap()
    }

    async fn delete_user(&mut self, user_id: i32) -> Result<u64, Error> {
        let conn = get_database_connection().await;

        let res = UserTable::delete_by_id(user_id).exec(conn).await.unwrap();

        Ok(res.rows_affected)
    }

    async fn make_payment(
        &mut self,
        user_id: i64,
        from: NaiveDateTime,
        to: NaiveDateTime,
    ) -> Option<user_table::Model> {
        // NOTE:
        // update taking record to paid
        // update user money
        // inser payment history
        // then insert money history, that state user is make payment
        // from date A to date B with amount of xxxx, and final user money is xxxx

        let conn = get_database_connection().await;

        let start_date = from.date().and_hms_opt(0, 0, 0).unwrap();
        let end_date = to
            .date()
            .checked_add_days(Days::new(1))
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();

        let user = self.user_table.get_by_id(user_id as i32).await.unwrap();

        if user.is_none() {
            return None;
        }

        let user = user.unwrap();

        let _update_taking_record = TakingRecordTable::update_many()
            .col_expr(taking_record_table::Column::IsPaid, Expr::value(true))
            .filter(taking_record_table::Column::TakenDate.gte(start_date))
            .filter(taking_record_table::Column::TakenDate.lt(end_date))
            .filter(taking_record_table::Column::UserId.eq(user_id))
            .exec(conn)
            .await
            .unwrap();

        let total_bill = TakingRecordTable::find()
            .filter(taking_record_table::Column::TakenDate.gte(start_date))
            .filter(taking_record_table::Column::TakenDate.lt(end_date))
            .filter(taking_record_table::Column::UserId.eq(user_id))
            .find_with_related(PriceHistoryTable)
            .all(conn)
            .await
            .unwrap()
            .into_iter()
            .flat_map(|(_, prices)| prices)
            .map(|prices| prices.price)
            .sum::<i64>();

        let final_user_money = user.money - total_bill;

        let _insert_money_history = self.money_history_table.create(money_history_table::ActiveModel {
            id: NotSet,
            user_id: Set(user_id),
            date: Set(Local::now().naive_local()),
            money_amount: Set(total_bill),
            description: Set(format!(
                "{0} is make payment from date {1} to date {2}, with total bill Rp.{3}. Rp.{4} => Rp.{5}",
                user.username,
                from.to_string(),
                to.to_string(),
                format_as_idr(total_bill),
                format_as_idr(user.money),
                format_as_idr(final_user_money)
            )),
        })
        .await
        .unwrap();

        let updated_user = self
            .user_table
            .update_by_model(user_table::ActiveModel {
                id: NotSet,
                username: NotSet,
                is_active: NotSet,
                is_admin: NotSet,
                money: Set(final_user_money),
                created_date: NotSet,
                updated_date: Set(Local::now().naive_local()),
            })
            .await
            .unwrap();

        Some(updated_user)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn command_add_money() {
        // let result = CustomerMoneyCommand::add_money(1, 4000000).await;
        // println!("result: {result:?}");
    }

    #[tokio::test]
    async fn command_get_all_user_money() {
        // let user_money = CustomerMoneyCommand::get_all_user_money_history(1).await;
        // println!("result: {user_money:?}");
    }
}
