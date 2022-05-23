use {crate::cli::CliError, cronos_sdk::Client, solana_sdk::pubkey::Pubkey};

pub fn get(client: &Client) -> Result<(), CliError> {
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
    let config_pubkey = cronos_sdk::scheduler::state::Config::pda().0;
    let data = client
        .get_account_data(&config_pubkey)
        .map_err(|_err| CliError::AccountNotFound(config_pubkey.to_string()))?;
    let config = cronos_sdk::scheduler::state::Config::try_from(data)
        .map_err(|_err| CliError::AccountDataNotParsable(config_pubkey.to_string()))?;

    let settings = cronos_sdk::scheduler::state::ConfigSettings {
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

    let ix = cronos_sdk::scheduler::instruction::admin_config_update(
        client.payer_pubkey(),
        config_pubkey,
        settings,
    );

    client.sign_and_submit(&[ix], &[client.payer()]).unwrap();

    Ok(())
}
