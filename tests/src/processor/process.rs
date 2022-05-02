use std::sync::Arc;

use clap::ArgMatches;
use solana_client_helpers::{Client, RpcClient};

use crate::{command::CliCommand, config::CliConfig, error::TestError, utils::load_keypair};

pub fn process(matches: &ArgMatches) -> Result<(), TestError> {
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
        CliCommand::AdminInitialize => super::admin::initialize(&client),
        CliCommand::Bench {
            count,
            time,
            percent,
        } => super::bench::test(count, time, percent),
    }
}
