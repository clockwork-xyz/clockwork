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
    env::var("es_cloud_id").unwrap()
}

pub fn es_user() -> String {
    env::var("es_user").unwrap()
}

pub fn es_password() -> String {
    env::var("es_password").unwrap()
}

pub fn es_health_index() -> String {
    env::var("es_health_index").unwrap()
}
