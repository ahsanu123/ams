use super::admin_user_seeds::SeedTrait;
use crate::model::user::{user_table, User, UserNoId};
use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods, RunQueryDsl, SqliteConnection};

#[derive(Default)]
pub struct UserSeed {}

impl SeedTrait for UserSeed {
    fn seed_db(&self, conn: &mut SqliteConnection) -> Result<bool, String> {
        let default_user = [
            UserNoId {
                username: "Tukimin".into(),
                is_active: true,
                money: 0.0,
                bill: 0.0,
                is_admin: false,
            },
            UserNoId {
                username: "Paijo".into(),
                is_active: true,
                money: 0.0,
                bill: 0.0,
                is_admin: false,
            },
            UserNoId {
                username: "Painem".into(),
                is_active: true,
                money: 0.0,
                bill: 0.0,
                is_admin: false,
            },
        ];

        default_user.iter().for_each(|item| {
            let user_data = user_table::table
                .filter(user_table::username.eq(item.username.clone()))
                .filter(user_table::is_admin.eq(false))
                .filter(user_table::is_active.eq(true))
                .first::<User>(conn);

            match user_data {
                Ok(_) => {}
                Err(_) => {
                    let _ = diesel::insert_into(user_table::table)
                        .values(item)
                        .execute(conn)
                        .expect("cant insert default user");
                }
            };
        });

        Ok(true)
    }
}
