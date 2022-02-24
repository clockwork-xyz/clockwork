use std::{str::FromStr, sync::Arc};

use anchor_client::ClientError;
use solana_client_helpers::Client;
use solana_sdk::{
    clock::{Clock, Epoch, Slot, UnixTimestamp},
    pubkey::Pubkey,
};

pub fn blocktime(client: &Arc<Client>) -> Result<i64, ClientError> {
    let clock = fetch_clock_sysvar(client).unwrap();
    Ok(clock.unix_timestamp)
}

fn fetch_clock_sysvar(client: &Arc<Client>) -> Result<Clock, ClientError> {
    let clock_addr = Pubkey::from_str("SysvarC1ock11111111111111111111111111111111").unwrap();
    let data = client.get_account_data(&clock_addr)?;
    Ok(get_clock_from_data(data))
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
