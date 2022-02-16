use std::env;

pub fn env_wss_endpoint() -> String {
    env::var("WSS_ENDPOINT").unwrap()
}
