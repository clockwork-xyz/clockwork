use {
    cronos_sdk::account::Health,
    dotenv::dotenv,
    solana_account_decoder::UiAccountEncoding,
    solana_client::{pubsub_client::PubsubClient, rpc_config::RpcAccountInfoConfig},
    solana_sdk::{account::Account, commitment_config::CommitmentConfig},
};

mod env;

fn main() {
    dotenv().ok();

    let (health_pubkey, _health_bump) = Health::pda();

    let (_ws_client, health_receiver) = PubsubClient::account_subscribe(
        env::wss_endpoint().as_str().into(),
        &health_pubkey,
        Some(RpcAccountInfoConfig {
            encoding: Some(UiAccountEncoding::Base64),
            commitment: Some(CommitmentConfig::processed()),
            data_slice: None,
        }),
    )
    .unwrap();

    for ui_account_response in health_receiver {
        let ui_account = ui_account_response.value;
        let account = ui_account.decode::<Account>().unwrap();
        let health = Health::try_from(account.data).unwrap();

        println!("last ping: {}", health.last_ping);
        println!("target ping: {}\n", health.target_ping);
    }
}
