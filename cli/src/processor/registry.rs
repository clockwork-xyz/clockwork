use {
    crate::errors::CliError,
    clockwork_client::{
        network::state::{Registry, Snapshot},
        Client,
    },
};

pub fn get(client: &Client) -> Result<(), CliError> {
    let registry_pubkey = clockwork_client::network::state::Registry::pubkey();
    let registry = client
        .get::<Registry>(&registry_pubkey)
        .map_err(|_err| CliError::AccountDataNotParsable(registry_pubkey.to_string()))?;

    let snapshot_pubkey = Snapshot::pubkey(registry.current_epoch);
    let snapshot = client
        .get::<Snapshot>(&snapshot_pubkey)
        .map_err(|_err| CliError::AccountDataNotParsable(snapshot_pubkey.to_string()))?;

    println!("{}\n{:#?}", registry_pubkey, registry);
    println!("{}\n{:#?}", snapshot_pubkey, snapshot);
    Ok(())
}

pub fn unlock(client: &Client) -> Result<(), CliError> {
    let ix = clockwork_client::network::instruction::registry_unlock(client.payer_pubkey());
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    get(client)?;
    Ok(())
}
