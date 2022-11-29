use clockwork_client::network::state::PoolSettings;

use {
    crate::errors::CliError,
    clockwork_client::{
        network::state::{Pool, Registry},
        Client,
    },
};

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
    let ix = clockwork_client::network::instruction::pool_update(
        client.payer_pubkey(),
        client.payer_pubkey(),
        pool_pubkey,
        PoolSettings { size },
    );
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    get(client, id)?;
    Ok(())
}
