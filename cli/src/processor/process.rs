use crate::{
    cli::{CliCommand, CliError},
    config::CliConfig,
};
use clap::ArgMatches;
use cronos_client::Client;
use solana_sdk::signature::read_keypair_file;

pub fn process(matches: &ArgMatches) -> Result<(), CliError> {
    // Parse command and config
    let command = CliCommand::try_from(matches)?;
    let config = CliConfig::load();

    // Build the RPC client
    let payer = read_keypair_file(config.keypair_path).unwrap();
    let client = Client::new(payer, config.json_rpc_url);

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
        CliCommand::NodeGet { delegate } => super::node::get_by_delegate(&client, delegate),
        CliCommand::NodeRegister { delegate } => super::node::register(&client, delegate),
        CliCommand::NodeStake { amount, delegate } => super::node::stake(&client, amount, delegate),
        CliCommand::PoolGet => super::pool::get(&client),
        CliCommand::ManagerCreate => super::manager::create(&client),
        CliCommand::ManagerGet { address } => super::manager::get(&client, &address),
        CliCommand::QueueCreate { schedule } => super::queue::create(&client, schedule),
        CliCommand::QueueGet { address } => super::queue::get(&client, &address),
        CliCommand::RegistryGet => super::registry::get(&client),
        CliCommand::SnapshotGet => super::snapshot::get(&client),
    }
}
