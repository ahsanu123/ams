use crate::repositories::user_repository::AdditionalUserTableMethodTrait;
use ams_entity::prelude::*;
use ams_entity::user_table;

use crate::repositories::abstract_repository_trait::AbstractRepository;

pub async fn insert_new_user(new_user: user_table::Model) -> i32 {
    let is_user_exits = UserTable::get_by_id(new_user.id).await.unwrap();

    if is_user_exits.is_some() {
        // user already exist
        return 0;
    }

    let active_model: user_table::ActiveModel = new_user.into();
    let result = UserTable::create(active_model).await.unwrap();

    result.id
}

pub async fn get_all_active_user() -> Vec<user_table::Model> {
    UserTable::get_all_active_user().await
}

#[cfg(test)]
mod test {

    #[test]
    fn insert_new_user() {
        todo!();
    }
}
