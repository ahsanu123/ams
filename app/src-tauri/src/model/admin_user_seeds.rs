use crate::model::user::{user_table, User, UserNoId};
use diesel::{query_dsl::methods::FilterDsl, ExpressionMethods, RunQueryDsl, SqliteConnection};

pub trait SeedTrait {
    fn seed_db(&self, conn: &mut SqliteConnection) -> Result<bool, String>;
}

#[derive(Default)]
pub struct AdminUserSeeds {}

impl SeedTrait for AdminUserSeeds {
    fn seed_db(&self, conn: &mut SqliteConnection) -> Result<bool, String> {
        let admin = user_table::table
            .filter(user_table::username.eq("Admin"))
            .filter(user_table::is_admin.eq(true))
            .filter(user_table::is_active.eq(true))
            .first::<User>(conn);

        match admin {
            Ok(_) => Ok(true),
            Err(_) => {
                let default_admin = UserNoId {
                    username: "Admin".into(),
                    is_active: true,
                    money: 0.0,
                    bill: 0.0,
                    is_admin: true,
                };

                let _ = diesel::insert_into(user_table::table)
                    .values(default_admin)
                    .execute(conn)
                    .expect("cant insert default admin");

                Ok(true)
            }
        }
    }
}
