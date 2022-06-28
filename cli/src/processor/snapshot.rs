use cronos_client::network::state::{Registry, Snapshot};

use {crate::cli::CliError, cronos_client::Client};

pub fn get(client: &Client) -> Result<(), CliError> {
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

    Ok(())
}
