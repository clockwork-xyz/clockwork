use {
    crate::errors::CliError,
    clockwork_client::{
        network::state::Config as NetworkConfig, pool::state::Config as PoolConfig,
        scheduler::state::Config as SchedulerConfig, Client,
    },
    solana_sdk::pubkey::Pubkey,
};

pub fn get(client: &Client) -> Result<(), CliError> {
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

    // Get scheduler config
    let scheduler_config_pubkey = SchedulerConfig::pubkey();
    let scheduler_config = client
        .get::<SchedulerConfig>(&scheduler_config_pubkey)
        .map_err(|_err| CliError::AccountNotFound(scheduler_config_pubkey.to_string()))?;

    // Print configs
    println!("Network {:#?}", network_config);
    println!("Pool {:#?}", pool_config);
    println!("Scheduler {:#?}", scheduler_config);

    Ok(())
}

pub fn set(
    client: &Client,
    admin: Option<Pubkey>,
    worker_fee: Option<u64>,
    grace_period: Option<i64>,
    spam_penalty: Option<u64>,
) -> Result<(), CliError> {
    let config_pubkey = SchedulerConfig::pubkey();
    let config = client
        .get::<SchedulerConfig>(&config_pubkey)
        .map_err(|_err| CliError::AccountNotFound(config_pubkey.to_string()))?;

    let settings = clockwork_client::scheduler::state::ConfigSettings {
        admin: match admin {
            Some(admin) => admin,
            None => config.admin,
        },
        worker_fee: match worker_fee {
            Some(worker_fee) => worker_fee,
            None => config.worker_fee,
        },
        grace_period: match grace_period {
            Some(grace_period) => grace_period,
            None => config.grace_period,
        },
        spam_penalty: match spam_penalty {
            Some(spam_penalty) => spam_penalty,
            None => config.spam_penalty,
        },
    };

    let ix = clockwork_client::scheduler::instruction::admin_config_update(
        client.payer_pubkey(),
        config_pubkey,
        settings,
    );

    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();

    Ok(())
}
