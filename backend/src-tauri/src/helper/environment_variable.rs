use dotenvy;
use std::sync::OnceLock;

pub static ENV_VAR: OnceLock<EnvironmentVariable> = OnceLock::new();

pub struct EnvironmentVariable {
    pub sqlite_connection_string: String,
    pub postgres_connection_string: String,
}

impl EnvironmentVariable {
    fn new() -> Self {
        Self {
            sqlite_connection_string: dotenvy::var("sqlite_connection_string").unwrap(),
            postgres_connection_string: dotenvy::var("postgres_connecttion_string").unwrap(),
        }
    }
}
