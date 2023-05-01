use anchor_lang::{
    solana_program::instruction::{AccountMeta, Instruction},
    InstructionData,
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
    // TODO
    // let ix = clockwork_network_program::instruction::registry_unlock(client.payer_pubkey());
    let ix = Instruction {
        program_id: clockwork_network_program::ID,
        accounts: vec![
            AccountMeta::new(client.payer_pubkey(), true),
            AccountMeta::new_readonly(Config::pubkey(), false),
            AccountMeta::new(Registry::pubkey(), false),
        ],
        data: clockwork_network_program::instruction::RegistryUnlock {}.data(),
    };
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    get(client)?;
    Ok(())
}
