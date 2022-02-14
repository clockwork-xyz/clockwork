use std::sync::Arc;

use clap::ArgMatches;
use solana_client_helpers::{Client, RpcClient};

use crate::{command::CliCommand, config::CliConfig, error::CliError, utils::load_keypair};

pub fn process(matches: &ArgMatches) -> Result<(), CliError> {
    let command = CliCommand::try_from(matches)?;
    let config = CliConfig::try_from(matches)?;

    let payer = load_keypair(&config);
    let client = RpcClient::new_with_timeouts_and_commitment(
        config.json_rpc_url.to_string(),
        config.rpc_timeout,
        config.commitment,
        config.confirm_transaction_initial_timeout,
    );
    let client = Arc::new(Client { client, payer });

    match command {
        CliCommand::Blocktime => super::blocktime::get(&client),
        CliCommand::DaemonNew => super::daemon::new(&client),
        CliCommand::DaemonData => super::daemon::data(&client),
        CliCommand::HealthCheck => super::health::check(&client),
        CliCommand::TaskData { address } => super::task::data(&client, &address),
        CliCommand::TaskNewMemo {
            memo,
            exec_at,
            stop_at,
            recurr,
        } => super::task::new(&client, memo, exec_at, stop_at, recurr),
    }
}
