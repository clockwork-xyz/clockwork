use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        system_program,
    },
    InstructionData,
};
use clockwork_network_program::state::{Config, Pool, Registry, PoolSettings};

use crate::{client::Client, errors::CliError};

pub fn get(client: &Client, id: u64) -> Result<(), CliError> {
    let pool_pubkey = Pool::pubkey(id);
    let pool = client
        .get::<Pool>(&pool_pubkey)
        .map_err(|_err| CliError::AccountDataNotParsable(pool_pubkey.to_string()))?;
    println!("{:#?}", pool);
    Ok(())
}

pub fn list(client: &Client) -> Result<(), CliError> {
    let registry_pubkey = Registry::pubkey();
    let registry = client
        .get::<Registry>(&registry_pubkey)
        .map_err(|_err| CliError::AccountDataNotParsable(registry_pubkey.to_string()))?;

    for pool_id in 0..registry.total_pools {
        let pool_pubkey = Pool::pubkey(pool_id);
        let pool = client
            .get::<Pool>(&pool_pubkey)
            .map_err(|_err| CliError::AccountDataNotParsable(pool_pubkey.to_string()))?;
        println!("{:#?}", pool);
    }

    Ok(())
}

pub fn update(client: &Client, id: u64, size: usize) -> Result<(), CliError> {
    let pool_pubkey = Pool::pubkey(id);
    // let ix = clockwork_client::network::instruction::pool_update(
    //     client.payer_pubkey(),
    //     client.payer_pubkey(),
    //     pool_pubkey,
    //     PoolSettings { size },
    // );
    let settings = PoolSettings { size };
    let ix = Instruction {
        program_id: clockwork_network_program::ID,
        accounts: vec![
            AccountMeta::new_readonly(client.payer_pubkey(), true),
            AccountMeta::new_readonly(Config::pubkey(), false),
            AccountMeta::new(client.payer_pubkey(), true),
            AccountMeta::new(pool_pubkey, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: clockwork_network_program::instruction::PoolUpdate { settings }.data(),
    };
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    get(client, id)?;
    Ok(())
}
