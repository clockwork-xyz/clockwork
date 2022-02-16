use std::env;

pub fn env_psql_params() -> String {
    env::var("PSQL_PARAMS").unwrap()
}
