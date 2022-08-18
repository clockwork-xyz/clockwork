use {
    crate::errors::CliError,
    clockwork_client::{
        crank::state::Config as CrankConfig, network::state::Config as NetworkConfig,
        pool::state::Config as PoolConfig, Client,
    },
    solana_sdk::pubkey::Pubkey,
};

pub fn get(client: &Client) -> Result<(), CliError> {
    // Get crank config
    let crank_config_pubkey = CrankConfig::pubkey();
    let crank_config = client
        .get::<CrankConfig>(&crank_config_pubkey)
        .map_err(|_err| CliError::AccountNotFound(crank_config_pubkey.to_string()))?;

    // Get network config
    let network_config_pubkey = NetworkConfig::pubkey();
    let network_config = client
        .get::<NetworkConfig>(&network_config_pubkey)
        .map_err(|_err| CliError::AccountNotFound(network_config_pubkey.to_string()))?;

    // Get pool config
    let pool_config_pubkey = PoolConfig::pubkey();
    let pool_config = client
        .get::<PoolConfig>(&pool_config_pubkey)
        .map_err(|_err| CliError::AccountNotFound(pool_config_pubkey.to_string()))?;

    // Print configs
    println!("Crank {:#?}", crank_config);
    println!("Network {:#?}", network_config);
    println!("Pool {:#?}", pool_config);

    Ok(())
}

pub fn set(
    client: &Client,
    admin: Option<Pubkey>,
    automation_fee: Option<u64>,
) -> Result<(), CliError> {
    let config_pubkey = CrankConfig::pubkey();
    let config = client
        .get::<CrankConfig>(&config_pubkey)
        .map_err(|_err| CliError::AccountNotFound(config_pubkey.to_string()))?;

    let settings = clockwork_client::crank::state::ConfigSettings {
        admin: match admin {
            Some(admin) => admin,
            None => config.admin,
        },
        automation_fee: match automation_fee {
            Some(automation_fee) => automation_fee,
            None => config.automation_fee,
        },
    };

    // let ix = clockwork_client::crank::instruction::admin_config_update(
    //     client.payer_pubkey(),
    //     config_pubkey,
    //     settings,
    // );

    // client.send_and_confirm(&[ix], &[client.payer()]).unwrap();

    Ok(())
}
