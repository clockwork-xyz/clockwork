use {
    crate::utils::{new_rpc_client, wss_endpoint},
    cronos_sdk::account::Health,
    dotenv::dotenv,
    solana_account_decoder::UiAccountEncoding,
    solana_client::{pubsub_client::PubsubClient, rpc_config::RpcAccountInfoConfig},
    solana_sdk::{account::Account, clock::*, commitment_config::CommitmentConfig, pubkey::Pubkey},
    std::str::FromStr,
};

mod utils;

fn main() {
    dotenv().ok();

    let client = new_rpc_client();

    let (health_pubkey, _health_bump) = Health::pda();
    let clock_addr = Pubkey::from_str("SysvarC1ock11111111111111111111111111111111").unwrap();

    let (_ws_client, clock_receiver) = PubsubClient::account_subscribe(
        wss_endpoint().as_str().into(),
        &clock_addr,
        Some(RpcAccountInfoConfig {
            encoding: Some(UiAccountEncoding::Base64),
            commitment: Some(CommitmentConfig::processed()),
            data_slice: None,
        }),
    )
    .unwrap();

    for ui_account_response in clock_receiver {
        let ui_account = ui_account_response.value;
        let account = ui_account.decode::<Account>().unwrap();
        let clock = get_clock_from_data(account.data);

        let health_account = client.get_account_data(&health_pubkey).unwrap();
        let health_data = Health::try_from(health_account).unwrap();

        let blocktime = clock.unix_timestamp;
        let last_ping = blocktime - health_data.last_ping;
        let recurr_drift = blocktime - health_data.target_ping;

        println!("   Blocktime: {}", blocktime);
        println!("   Last ping: {} sec", last_ping);
        println!("Recurr drift: {} sec\n", recurr_drift);
    }
}

fn get_clock_from_data(data: Vec<u8>) -> Clock {
    Clock {
        slot: Slot::from_le_bytes(data.as_slice()[0..8].try_into().unwrap()),
        epoch_start_timestamp: UnixTimestamp::from_le_bytes(
            data.as_slice()[8..16].try_into().unwrap(),
        ),
        epoch: Epoch::from_le_bytes(data.as_slice()[16..24].try_into().unwrap()),
        leader_schedule_epoch: Epoch::from_le_bytes(data.as_slice()[24..32].try_into().unwrap()),
        unix_timestamp: UnixTimestamp::from_le_bytes(data.as_slice()[32..40].try_into().unwrap()),
    }
}
