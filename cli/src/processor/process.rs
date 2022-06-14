use crate::{
    cli::{CliCommand, CliError},
    config::CliConfig,
};
use clap::ArgMatches;
use cronos_client::Client;

pub fn process(matches: &ArgMatches) -> Result<(), CliError> {
    // Parse command and config
    let command = CliCommand::try_from(matches)?;
    let config = CliConfig::load();

    // Build the RPC client
    let client = Client::new(
        config.keypair_path,
        config.json_rpc_url,
        config.websocket_url,
    );

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
