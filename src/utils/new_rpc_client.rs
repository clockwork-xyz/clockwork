use {
    crate::env,
    solana_client_helpers::{Client, RpcClient},
    solana_sdk::{commitment_config::CommitmentConfig, signature::read_keypair},
    std::fs::File,
};

pub fn new_rpc_client() -> Client {
    let payer = read_keypair(&mut File::open(env::keypath().as_str()).unwrap()).unwrap();
    let client = RpcClient::new_with_commitment::<String>(
        env::rpc_endpoint().as_str().into(),
        CommitmentConfig::confirmed(),
    );
    Client { client, payer }
}
