use std::sync::Arc;

use clap::ArgMatches;
use solana_client_helpers::{Client, RpcClient};

use crate::{command::CliCommand, config::CliConfig, error::CliError, utils::load_keypair};

pub fn process(matches: &ArgMatches) -> Result<(), CliError> {
    // Parse command and config
    let command = CliCommand::try_from(matches)?;
    let config = CliConfig::load();

    // Build the RPC client
    let payer = load_keypair(&config);
    let client = RpcClient::new_with_timeouts_and_commitment(
        config.json_rpc_url.to_string(),
        config.rpc_timeout,
        config.commitment,
        config.confirm_transaction_initial_timeout,
    );
    let client = Arc::new(Client { client, payer });

    // Process the command
    match command {
        // CliCommand::AdminTaskClose { address } => super::admin::task_close(&client, &address),
        // CliCommand::AdminHealthReset => super::admin::health_reset(&client),
        CliCommand::AdminOpen => super::admin::open(&client),
        // CliCommand::Clock => super::clock::get(&client),
        // CliCommand::ConfigGet => super::config::get(&client),
        // CliCommand::DaemonOpen => super::daemon::open(&client),
        // CliCommand::DaemonGet => super::daemon::get(&client),
        // CliCommand::HealthGet => super::health::get(&client),
        // CliCommand::TaskClose { address } => super::task::close(&client, &address),
        // CliCommand::TaskGet { address } => super::task::get(&client, &address),
        // CliCommand::TaskOpen { ix, schedule } => super::task::open(&client, ix, schedule),
        _ => Ok(()),
    }
}
