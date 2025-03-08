use dotenvy::{self, dotenv};
use once_cell::sync::Lazy;

pub static ENV_VAR: Lazy<EnvironmentVariable> = Lazy::new(|| {
    dotenv().ok();
    EnvironmentVariable::new()
});

pub struct EnvironmentVariable {
    pub sqlite_connection_string: String,
    pub postgres_connection_string: String,
}

impl EnvironmentVariable {
    fn new() -> Self {
        Self {
            sqlite_connection_string: dotenvy::var("sqlite_connection_string")
                .expect("sqlite connection must provided!!!"),
            postgres_connection_string: dotenvy::var("postgres_connection_string")
                .expect("postgresql connection must provided!!!"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_env_variable() {
        println!("{}", ENV_VAR.sqlite_connection_string);
        println!("{}", ENV_VAR.postgres_connection_string);
    }
}
