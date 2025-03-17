use super::environment_variable::ENV_VAR;
use diesel::{Connection, SqliteConnection};

pub fn create_connection() -> SqliteConnection {
    let connection_string = &ENV_VAR.sqlite_connection_string;
    SqliteConnection::establish(&connection_string)
        .unwrap_or_else(|_| panic!("Error connecting to {}", connection_string))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_connection() {
        let connection_string = &ENV_VAR.sqlite_connection_string;
        let conn = create_connection();
        println!("Success connecting to database, {connection_string}");
    }
}
