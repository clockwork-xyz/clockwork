use anchor_lang::{
    solana_program::{
        instruction::Instruction, system_program,
    },
    InstructionData, ToAccountMetas
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
    let settings = PoolSettings { size };
    let ix = Instruction {
        program_id: clockwork_network_program::ID,
        accounts: clockwork_network_program::accounts::PoolUpdate {
            admin: client.payer_pubkey(),
            config: Config::pubkey(),
            payer: client.payer_pubkey(),
            pool: pool_pubkey,
            system_program: system_program::ID,
        }.to_account_metas(Some(false)),
        data: clockwork_network_program::instruction::PoolUpdate { settings }.data(),
    };
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    get(client, id)?;
    Ok(())
}
