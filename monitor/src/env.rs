use std::env;

pub fn wss_endpoint() -> String {
    env::var("WSS_ENDPOINT").unwrap()
}
