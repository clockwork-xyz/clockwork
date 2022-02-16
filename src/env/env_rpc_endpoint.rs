use std::env;

pub fn env_rpc_endpoint() -> String {
    env::var("RPC_ENDPOINT").unwrap()
}
