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
        CliCommand::TaskGet { address } => super::task::get(&client, &address),
        CliCommand::Clock => super::clock::get(&client),
        CliCommand::ConfigGet => super::config::get(&client),
        CliCommand::ConfigSet {
            admin,
            delegate_fee,
            delegate_holdout_period,
            delegate_spam_penalty,
            program_fee,
        } => super::config::set(
            &client,
            admin,
            delegate_fee,
            delegate_holdout_period,
            delegate_spam_penalty,
            program_fee,
        ),
        CliCommand::Health => super::health::get(&client),
        CliCommand::Initialize { mint } => super::initialize::initialize(&client, mint),
        CliCommand::NodeRegister { delegate } => super::node::register(&client, delegate),
        CliCommand::NodeStake { amount, delegate } => super::node::stake(&client, amount, delegate),
        CliCommand::PoolGet => super::pool::get(&client),
        CliCommand::ManagerCreate => super::manager::create(&client),
        CliCommand::ManagerGet { address } => super::manager::get(&client, &address),
        CliCommand::QueueCreate { schedule } => super::queue::create(&client, schedule),
        CliCommand::QueueGet { address } => super::queue::get(&client, &address),
    }
}
