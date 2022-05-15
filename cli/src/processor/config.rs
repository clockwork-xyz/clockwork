use {crate::cli::CliError, solana_client_helpers::Client, std::sync::Arc};

pub fn get(client: &Arc<Client>) -> Result<(), CliError> {
    // Get heartbeat config
    let config_pubkey = cronos_sdk::heartbeat::state::Config::pda().0;
    let data = client
        .get_account_data(&config_pubkey)
        .map_err(|_err| CliError::AccountNotFound(config_pubkey.to_string()))?;
    let heartbeat_config = cronos_sdk::heartbeat::state::Config::try_from(data)
        .map_err(|_err| CliError::AccountDataNotParsable(config_pubkey.to_string()))?;

    // Get network config
    let config_pubkey = cronos_sdk::network::state::Config::pda().0;
    let data = client
        .get_account_data(&config_pubkey)
        .map_err(|_err| CliError::AccountNotFound(config_pubkey.to_string()))?;
    let network_config = cronos_sdk::network::state::Config::try_from(data)
        .map_err(|_err| CliError::AccountDataNotParsable(config_pubkey.to_string()))?;

    // Get pool config
    let config_pubkey = cronos_sdk::pool::state::Config::pda().0;
    let data = client
        .get_account_data(&config_pubkey)
        .map_err(|_err| CliError::AccountNotFound(config_pubkey.to_string()))?;
    let pool_config = cronos_sdk::pool::state::Config::try_from(data)
        .map_err(|_err| CliError::AccountDataNotParsable(config_pubkey.to_string()))?;

    // Get scheduler config
    let config_pubkey = cronos_sdk::scheduler::state::Config::pda().0;
    let data = client
        .get_account_data(&config_pubkey)
        .map_err(|_err| CliError::AccountNotFound(config_pubkey.to_string()))?;
    let scheduler_config = cronos_sdk::scheduler::state::Config::try_from(data)
        .map_err(|_err| CliError::AccountDataNotParsable(config_pubkey.to_string()))?;

    // Print configs
    println!("Heartbeat {:#?}", heartbeat_config);
    println!("Network {:#?}", network_config);
    println!("Pool {:#?}", pool_config);
    println!("Scheduler {:#?}", scheduler_config);

    Ok(())
}
