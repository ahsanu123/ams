use super::crud_repository_trait::CrudRepositoryTrait;
use crate::helper::sql_connection_helper::create_connection;
use crate::model::user::{user_table, UserNoId};
use crate::model::{user::User, *};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

pub struct UserRepository {}

impl CrudRepositoryTrait<User> for UserRepository {
    fn getAll(&self) -> Result<Vec<User>, String> {
        let conn = &mut create_connection();
        let result = user_table::table.load(conn);

        match result {
            Ok(users) => Ok(users),
            Err(_) => Err(format!("fail to users, {}, {}", file!(), line!())),
        }
    }

    fn create(&self, data: &User) -> Result<usize, String> {
        let conn = &mut create_connection();
        let insert_data: UserNoId = data.into();
        let result = diesel::insert_into(user_table::table)
            .values(insert_data)
            .execute(conn);

        match result {
            Ok(inserted_row) => Ok(inserted_row),
            Err(_) => Err(format!("fail to insert new user, {}, {}", file!(), line!())),
        }
    }

    fn read(&self, id: u32) -> Result<User, String> {
        let conn = &mut create_connection();
        let result = user_table::table
            .filter(user_table::id.eq(id as i32))
            .first(conn);

        match result {
            Ok(user) => Ok(user),
            Err(_) => Err(format!(
                "fail to get user with id {id}, {}, {}",
                file!(),
                line!()
            )),
        }
    }

    fn update(&self, data: &User) -> Result<usize, String> {
        let conn = &mut create_connection();
        let update_data: UserNoId = data.into();
        let update_query = user_table::table.filter(user_table::id.eq(data.id));

        let result = diesel::update(update_query).set(update_data).execute(conn);

        match result {
            Ok(updated_row) => Ok(updated_row),
            Err(_) => Err(format!("fail to update user, {}, {}", file!(), line!())),
        }
    }

    fn delete(&self, id: u32) -> Result<usize, String> {
        let conn = &mut create_connection();
        let delete_query = user_table::table.filter(user_table::id.eq(id as i32));

        let result = diesel::delete(delete_query).execute(conn);

        match result {
            Ok(deleted_row) => Ok(deleted_row),
            Err(_) => Err(format!("fail to delete user, {}, {}", file!(), line!())),
        }
    }
}

impl Default for UserRepository {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn user_repository_get_all() {
        todo!();
    }

    #[test]
    fn user_repository_create() {
        todo!();
    }

    #[test]
    fn user_repository_read() {
        let user_repo = UserRepository::default();
        let user_with_id_two = user_repo.read(2).expect("user with id 2 is not found");

        println!("here is user with id 2 {user_with_id_two:#?}");
    }

    #[test]
    fn user_repository_update() {
        todo!();
    }

    #[test]
    fn user_repository_delete() {
        todo!();
    }
}
