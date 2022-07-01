use cronos_client::network::state::{Registry, Snapshot, SnapshotEntry};
use solana_sdk::pubkey::Pubkey;

use {crate::errors::CliError, cronos_client::Client};

pub fn get(client: &Client, entry_id: Option<u64>) -> Result<(), CliError> {
    let registry_pubkey = cronos_client::network::state::Registry::pda().0;
    let registry = client
        .get::<Registry>(&registry_pubkey)
        .map_err(|_err| CliError::AccountDataNotParsable(registry_pubkey.to_string()))?;

    let snapshot_pubkey =
        cronos_client::network::state::Snapshot::pda(registry.snapshot_count - 1).0;
    let snapshot = client
        .get::<Snapshot>(&snapshot_pubkey)
        .map_err(|_err| CliError::AccountDataNotParsable(snapshot_pubkey.to_string()))?;

    println!("{:#?}", snapshot);

    match entry_id {
        None => (),
        Some(entry_id) => {
            get_snapshot_entry(client, snapshot_pubkey, entry_id).ok();
        }
    }

    Ok(())
}

pub fn get_snapshot_entry(
    client: &Client,
    snapshot_pubkey: Pubkey,
    entry_id: u64,
) -> Result<(), CliError> {
    let entry_pubkey =
        cronos_client::network::state::SnapshotEntry::pda(snapshot_pubkey, entry_id).0;

    let entry = client
        .get::<SnapshotEntry>(&entry_pubkey)
        .map_err(|_err| CliError::AccountDataNotParsable(snapshot_pubkey.to_string()))?;

    println!("{:#?}", entry);

    Ok(())
}
