// reference: https://docs.rs/argon2/latest/argon2/

use crate::model::hash::{hash_table, Hash, HashNoId};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, SaltString},
    Argon2, PasswordVerifier,
};
use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods, RunQueryDsl, SqliteConnection};

pub fn create_password_hash(
    user_id: &i32,
    password: &String,
    conn: &mut SqliteConnection,
) -> Result<(), String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();

    let hash_data = HashNoId {
        user_id: *user_id,
        hash: password_hash.clone(),
    };

    let user_hash = hash_table::table
        .filter(hash_table::user_id.eq(user_id))
        .first::<Hash>(conn);

    match user_hash {
        Ok(hash_in_db) => {
            let update_query = hash_table::table
                .filter(hash_table::id.eq(hash_in_db.id))
                .filter(hash_table::user_id.eq(user_id));

            let new_hash_data = HashNoId {
                user_id: hash_in_db.user_id,
                hash: password_hash.clone(),
            };

            let _ = diesel::update(update_query)
                .set(new_hash_data)
                .execute(conn);
            Ok(())
        }
        Err(_) => {
            let _ = diesel::insert_into(hash_table::table)
                .values(hash_data)
                .execute(conn)
                .expect("cant insert new hash data for user");

            Err(String::from("cant insert new hash data for user"))
        }
    }

    // NOTE: use this to debug output of password hash
    // println!("hash password of {password} => {password_hash:#?}");
}

pub fn validate_password(
    user_id: &i32,
    password: &String,
    conn: &mut SqliteConnection,
) -> Result<bool, String> {
    let hash_in_db = hash_table::table
        .filter(hash_table::user_id.eq(user_id))
        .first::<Hash>(conn)
        .expect("cant get hash for current user");

    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(&hash_in_db.hash).unwrap();

    let is_password_correct = argon2.verify_password(password.as_bytes(), &parsed_hash);

    match is_password_correct {
        Ok(_) => Ok(true),
        Err(_) => Err(String::from("Password Incorrect!!")),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::helper::sql_connection_helper::create_connection;

    #[test]
    fn test_create_password_hash() {
        let conn = &mut create_connection();
        let user_id = 1;

        let result = create_password_hash(&user_id, &String::from("supersecret"), conn);

        println!("create password hash result: {result:?}");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_password() {
        let conn = &mut create_connection();
        let password = String::from("supersecret");
        let user_id = 1;

        let result = validate_password(&user_id, &password, conn);

        println!("hashing success? {result:?}");
        assert!(result.is_ok());
    }
}
