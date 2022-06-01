use {
    crate::cli::CliError,
    cronos_client::{
        network::state::Config as NetworkConfig, pool::state::Config as PoolConfig,
        scheduler::state::Config as SchedulerConfig, Client,
    },
    solana_sdk::pubkey::Pubkey,
};

pub fn get(client: &Client) -> Result<(), CliError> {
    // Get network config
    let network_config_pubkey = NetworkConfig::pda().0;
    let network_config = client
        .get::<NetworkConfig>(&network_config_pubkey)
        .map_err(|_err| CliError::AccountNotFound(network_config_pubkey.to_string()))?;

    // Get pool config
    let pool_config_pubkey = PoolConfig::pda().0;
    let pool_config = client
        .get::<PoolConfig>(&pool_config_pubkey)
        .map_err(|_err| CliError::AccountNotFound(pool_config_pubkey.to_string()))?;

    // Get scheduler config
    let scheduler_config_pubkey = SchedulerConfig::pda().0;
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
    delegate_fee: Option<u64>,
    delegate_holdout_period: Option<i64>,
    delegate_spam_penalty: Option<u64>,
    program_fee: Option<u64>,
) -> Result<(), CliError> {
    let config_pubkey = SchedulerConfig::pda().0;
    let config = client
        .get::<SchedulerConfig>(&config_pubkey)
        .map_err(|_err| CliError::AccountNotFound(config_pubkey.to_string()))?;

    let settings = cronos_client::scheduler::state::ConfigSettings {
        admin: match admin {
            Some(admin) => admin,
            None => config.admin,
        },
        delegate_fee: match delegate_fee {
            Some(delegate_fee) => delegate_fee,
            None => config.delegate_fee,
        },
        delegate_holdout_period: match delegate_holdout_period {
            Some(delegate_holdout_period) => delegate_holdout_period,
            None => config.delegate_holdout_period,
        },
        delegate_spam_penalty: match delegate_spam_penalty {
            Some(delegate_spam_penalty) => delegate_spam_penalty,
            None => config.delegate_spam_penalty,
        },
        program_fee: match program_fee {
            Some(program_fee) => program_fee,
            None => config.program_fee,
        },
    };

    let ix = cronos_client::scheduler::instruction::admin_config_update(
        client.payer_pubkey(),
        config_pubkey,
        settings,
    );

    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();

    Ok(())
}
