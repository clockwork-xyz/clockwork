use std::env;

pub fn env_keypath() -> String {
    env::var("KEYPATH").unwrap()
}
