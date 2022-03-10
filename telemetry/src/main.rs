use {
    cronos_sdk::account::Health,
    dotenv::dotenv,
    solana_client_helpers::Client,
    solana_client_helpers::RpcClient,
    solana_sdk::{commitment_config::CommitmentConfig, signature::read_keypair},
    std::{fs::File, sync::Arc},
};

mod env;

fn main() {
    dotenv().ok();
    let client = &Arc::new(new_client());
    let (health_pubkey, _health_bump) = Health::pda();
    let time_receiver = cronos_sdk::clock::get_time(client);
    for ts in time_receiver {
        let health_account = client.get_account_data(&health_pubkey).unwrap();
        let health_data = Health::try_from(health_account).unwrap();

        let last_ping = ts - health_data.last_ping;
        let recurr_drift = ts - health_data.target_ping;

        println!("       Clock: {}", ts);
        println!("   Last ping: {} sec", last_ping);
        println!("Recurr drift: {} sec\n", recurr_drift);
    }
}

fn new_client() -> Client {
    let payer = read_keypair(&mut File::open(env::keypath().as_str()).unwrap()).unwrap();
    let client = RpcClient::new_with_commitment::<String>(
        env::rpc_endpoint().as_str().into(),
        CommitmentConfig::processed(),
    );
    Client { client, payer }
}
