use std::env;

pub fn keypath() -> String {
    env::var("KEYPATH").unwrap()
}

pub fn wss_endpoint() -> String {
    env::var("WSS_ENDPOINT").unwrap()
}

pub fn rpc_endpoint() -> String {
    env::var("RPC_ENDPOINT").unwrap()
}

pub fn es_cloud_id() -> String {
    env::var("ES_CLOUD_ID").unwrap()
}

pub fn es_user() -> String {
    env::var("ES_USER").unwrap()
}

pub fn es_password() -> String {
    env::var("ES_PASSWORD").unwrap()
}

pub fn es_health_index() -> String {
    env::var("ES_HEALTH_INDEX").unwrap()
}
