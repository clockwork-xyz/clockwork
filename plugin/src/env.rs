use std::env;

pub fn keypath() -> String {
    env::var("KEYPATH").unwrap()
}

pub fn rpc_endpoint() -> String {
    env::var("RPC_ENDPOINT").unwrap()
}
