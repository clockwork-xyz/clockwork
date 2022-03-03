use anchor_client::ClientError;
use solana_account_decoder::UiAccountEncoding;
use solana_client::{pubsub_client::PubsubClient, rpc_config::RpcAccountInfoConfig};
use solana_client_helpers::Client;
use solana_sdk::{
    account::Account,
    clock::{Clock, Epoch, Slot, UnixTimestamp},
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
};
use std::{
    str::FromStr,
    sync::{
        mpsc::{self, Receiver},
        Arc,
    },
    thread,
};

pub fn get_blocktime(client: &Arc<Client>) -> Result<i64, ClientError> {
    let clock = fetch_clock_sysvar(client).unwrap();
    Ok(clock.unix_timestamp)
}

pub fn monitor_blocktime(url: String) -> Receiver<i64> {
    let (blocktime_sender, blocktime_receiver) = mpsc::channel::<i64>();
    thread::spawn(move || {
        let mut latest_blocktime: i64 = 0;
        let (_ws_client, clock_receiver) = PubsubClient::account_subscribe(
            url.as_str(),
            &clock_sysvar_address(),
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
            let clock = deserialize_clock_sysvar(account.data);
            let blocktime = clock.unix_timestamp;
            if blocktime > latest_blocktime {
                latest_blocktime = blocktime;
                blocktime_sender.send(blocktime).unwrap()
            }
        }
    });
    return blocktime_receiver;
}

fn clock_sysvar_address() -> Pubkey {
    Pubkey::from_str("SysvarC1ock11111111111111111111111111111111").unwrap()
}

fn fetch_clock_sysvar(client: &Arc<Client>) -> Result<Clock, ClientError> {
    let data = client.get_account_data(&clock_sysvar_address())?;
    Ok(deserialize_clock_sysvar(data))
}

fn deserialize_clock_sysvar(data: Vec<u8>) -> Clock {
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
