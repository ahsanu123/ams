use once_cell::sync::OnceCell;

pub static ENV_VAR: OnceCell<EnvironmentVariable> = OnceCell::new();

#[derive(Debug)]
pub struct EnvironmentVariable {
    pub sqlite_connection_string: String,
}

#[cfg(test)]
mod helper_env_test {
    use super::*;

    #[test]
    fn check_env_variable() {
        println!("{}", ENV_VAR.get().unwrap().sqlite_connection_string);
    }
}
