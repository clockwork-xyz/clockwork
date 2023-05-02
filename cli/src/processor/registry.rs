use anchor_lang::{
    solana_program::instruction::Instruction,
    InstructionData, ToAccountMetas
};
use clockwork_network_program::state::{Config, Registry, Snapshot};

use crate::{client::Client, errors::CliError};

pub fn get(client: &Client) -> Result<(), CliError> {
    let registry_pubkey = clockwork_network_program::state::Registry::pubkey();
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
    let ix = Instruction {
        program_id: clockwork_network_program::ID,
        accounts: clockwork_network_program::accounts::RegistryUnlock {
            admin: client.payer_pubkey(),
            config: Config::pubkey(),
            registry: Registry::pubkey()
        }.to_account_metas(Some(false)),
        data: clockwork_network_program::instruction::RegistryUnlock {}.data(),
    };
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    get(client)?;
    Ok(())
}
