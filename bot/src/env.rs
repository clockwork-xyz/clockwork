use std::env;

pub fn keypath() -> String {
    env::var("KEYPATH").unwrap()
}

pub fn psql_params() -> String {
    env::var("PSQL_PARAMS").unwrap()
}

pub fn rpc_endpoint() -> String {
    env::var("RPC_ENDPOINT").unwrap()
}

pub fn wss_endpoint() -> String {
    env::var("WSS_ENDPOINT").unwrap()
}
