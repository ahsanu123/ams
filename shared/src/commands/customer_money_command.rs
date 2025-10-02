use ams_entity::{money_history_table, prelude::*, user_table};
use chrono::Local;
use sea_orm::{
    ActiveValue::{NotSet, Set},
    EntityTrait, QueryFilter,
    entity::*,
};
use std::fmt::Error;

use crate::repositories::{
    abstract_repository_trait::AbstractRepository, get_sql_connection_trait::GetSqlConnectionTrait,
};

pub async fn add_money(user_id: i64, amount: i64) -> Result<user_table::Model, Error> {
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

pub async fn get_all_user_money_history(user_id: i64) -> Vec<money_history_table::Model> {
    let conn = MoneyHistoryTable::get_connection().await;

    let datas = MoneyHistoryTable::find()
        .filter(money_history_table::Column::UserId.eq(user_id))
        .all(conn)
        .await
        .unwrap();

    datas
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn command_add_money() {
        let result = add_money(1, 4000000).await;

        println!("result: {result:?}");
    }

    #[tokio::test]
    async fn command_get_all_user_money() {
        let user_money = get_all_user_money_history(1).await;

        println!("result: {user_money:?}");
    }
}
