use dotenvy::{self, dotenv};
use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;

pub static ENV_VAR: OnceCell<EnvironmentVariable> = OnceCell::new();

// pub static ENV_VAR: Lazy<EnvironmentVariable> = Lazy::new(|| {
//     dotenv().ok();
//     EnvironmentVariable::new()
// });

#[derive(Debug)]
pub struct EnvironmentVariable {
    pub sqlite_connection_string: String,
}

impl EnvironmentVariable {
    pub fn new() -> Self {
        Self {
            sqlite_connection_string: dotenvy::var("sqlite_connection_string")
                .expect("sqlite connection must provided!!!"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_env_variable() {
        println!("{}", ENV_VAR.get().unwrap().sqlite_connection_string);
    }
}
