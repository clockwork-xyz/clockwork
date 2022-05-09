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
        CliCommand::Initialize { mint } => super::initialize::initialize(&client, mint),
        CliCommand::Clock => super::clock::get(&client),
        CliCommand::Config => super::config::get(&client),
        CliCommand::Health => super::health::get(&client),
        CliCommand::NodeRegister => super::node::register(&client),
        CliCommand::NodeStake { amount } => super::node::stake(&client, amount),
        CliCommand::QueueCreate => super::queue::create(&client),
        CliCommand::QueueGet { address } => super::queue::get(&client, &address),
        CliCommand::TaskCancel { address } => super::task::cancel(&client, &address),
        CliCommand::TaskCreate { ix, schedule } => super::task::create(&client, ix, schedule),
        CliCommand::TaskGet { address } => super::task::get(&client, &address),
    }
}
