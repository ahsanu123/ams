use crate::repositories::{
    abstract_repository_trait::AbstractRepository, get_sql_connection_trait::GetSqlConnectionTrait,
};
use ams_entity::{money_history_table, prelude::*, taking_record_table, user_table};
use chrono::{Days, Local, NaiveDateTime};
use sea_orm::{
    ActiveValue::{NotSet, Set},
    EntityTrait, QueryFilter,
    entity::*,
    prelude::{Expr, async_trait},
};
use std::fmt::Error;

#[async_trait::async_trait]
pub trait CustomerMoneyCommandTrait {
    async fn add_money(user_id: i64, amount: i64) -> Result<user_table::Model, Error>;
    async fn get_all_user_money_history(user_id: i64) -> Vec<money_history_table::Model>;
    async fn make_payment(
        user_id: i64,
        from: NaiveDateTime,
        to: NaiveDateTime,
    ) -> Option<user_table::Model>;
}

pub struct CustomerMoneyCommand;

#[async_trait::async_trait]
impl CustomerMoneyCommandTrait for CustomerMoneyCommand {
    async fn add_money(user_id: i64, amount: i64) -> Result<user_table::Model, Error> {
        let user = UserTable::get_by_id(user_id as i32).await.unwrap();

        if user.is_none() {
            return Err(Error);
        }

        let user = user.unwrap();

        let mut active_model: user_table::ActiveModel = user.clone().into();
        active_model.money = Set(user.money + amount);

        // update user money
        let updated_user = UserTable::update_by_model(active_model).await.unwrap();

        // insert history to money history
        let money_history = money_history_table::ActiveModel {
            id: NotSet,
            user_id: Set(user_id),
            date: Set(Local::now().naive_local()),
            money_amount: Set(amount),
            description: Set(format!(
                "Add Money {0}, final money amount {1}",
                amount, updated_user.money
            )
            .into()),
        };

        let _ = MoneyHistoryTable::create(money_history).await.unwrap();

        Ok(updated_user)
    }

    async fn get_all_user_money_history(user_id: i64) -> Vec<money_history_table::Model> {
        let conn = MoneyHistoryTable::get_connection().await;

        let datas = MoneyHistoryTable::find()
            .filter(money_history_table::Column::UserId.eq(user_id))
            .all(conn)
            .await
            .unwrap();

        datas
    }

    async fn make_payment(
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

        let conn = TakingRecordTable::get_connection().await;

        let start_date = from.date().and_hms_opt(0, 0, 0).unwrap();
        let end_date = to
            .date()
            .checked_add_days(Days::new(1))
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();

        let user = UserTable::get_by_id(user_id as i32).await.unwrap();

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

        let _insert_money_history = MoneyHistoryTable::create(money_history_table::ActiveModel {
            id: NotSet,
            user_id: Set(user_id),
            date: Set(Local::now().naive_local()),
            money_amount: Set(total_bill),
            description: Set(format!(
                "{0} is make payment from date {1} to date {2}, with total bill {3}. {4} => {5}",
                user.username,
                from.to_string(),
                to.to_string(),
                total_bill,
                user.money,
                final_user_money
            )),
        })
        .await
        .unwrap();

        let updated_user = UserTable::update_by_model(user_table::ActiveModel {
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
        let result = CustomerMoneyCommand::add_money(1, 4000000).await;
        println!("result: {result:?}");
    }

    #[tokio::test]
    async fn command_get_all_user_money() {
        let user_money = CustomerMoneyCommand::get_all_user_money_history(1).await;
        println!("result: {user_money:?}");
    }
}
