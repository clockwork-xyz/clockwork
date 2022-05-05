use crate::{
    cli::{CliCommand, CliError},
    config::CliConfig,
    utils::load_keypair,
};
use clap::ArgMatches;
use solana_client_helpers::{Client, RpcClient};
use std::sync::Arc;

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
        // CliCommand::AdminTaskCancel { address } => super::admin::task_cancel(&client, &address),
        // CliCommand::AdminHealthReset => super::admin::health_reset(&client),
        CliCommand::Initialize => super::initialize::initialize(&client),
        CliCommand::Clock => super::clock::get(&client),
        CliCommand::Config => super::config::get(&client),
        CliCommand::DaemonCreate => super::daemon::create(&client),
        CliCommand::DaemonGet { address } => super::daemon::get(&client, &address),
        CliCommand::Health => super::health::get(&client),
        CliCommand::TaskCancel { address } => super::task::cancel(&client, &address),
        CliCommand::TaskCreate { ix, schedule } => super::task::create(&client, ix, schedule),
        CliCommand::TaskGet { address } => super::task::get(&client, &address),
    }
}
