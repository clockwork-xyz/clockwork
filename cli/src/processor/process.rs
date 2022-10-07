use crate::{cli::CliCommand, config::CliConfig, errors::CliError};
use clap::ArgMatches;
use clockwork_client::Client;
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
        CliCommand::ApiNew {
            ack_authority,
            base_url,
        } => super::api::api_new(&client, ack_authority, base_url),
        CliCommand::ConfigGet => super::config::get(&client),
        CliCommand::HttpRequestNew {
            api,
            id,
            method,
            route,
        } => super::webhook::request_new(&client, api, id, method, route),
        CliCommand::Initialize { mint } => super::initialize::initialize(&client, mint),
        CliCommand::Localnet { program_infos } => super::localnet::start(&client, program_infos),
        CliCommand::PoolGet => super::pool::get(&client),
        CliCommand::QueueGet { address } => super::queue::get(&client, &address),
        CliCommand::QueueUpdate {
            address,
            rate_limit,
        } => super::queue::update(&client, &address, rate_limit),
        CliCommand::RegistryGet => super::registry::get(&client),
        CliCommand::WorkerCreate { signatory } => super::worker::create(&client, signatory),
        CliCommand::WorkerDelegateStake {
            amount,
            worker_pubkey,
        } => super::worker::delegate_stake(&client, amount, worker_pubkey),
    }
}
