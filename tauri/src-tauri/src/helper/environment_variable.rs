use dotenvy;
use std::sync::OnceLock;

pub static ENV_VAR: OnceLock<EnvironmentVariable> = EnvironmentVariable::default();

pub struct EnvironmentVariable {
    pub sqlite_connection_string: String,
    pub postgres_connection_string: String,
}

impl Default for EnvironmentVariable {
    fn default() -> Self {
        Self {
            sqlite_connection_string: dotenvy::var("sqlite_connection_string")?,
            postgres_connection_string: dotenvy::var("postgres_connecttion_string")?,
        }
    }
}
