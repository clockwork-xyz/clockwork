use crate::{cli::CliCommand, config::CliConfig, errors::CliError};
use anyhow::Result;
use clap::ArgMatches;
use clockwork_client::Client;
use solana_sdk::signature::read_keypair_file;

pub fn process(matches: &ArgMatches) -> Result<(), CliError> {
    // Parse command and config
    let command = CliCommand::try_from(matches)?;

    match command {
        // Set solana config if using localnet command
        CliCommand::Localnet { program_infos: _ } => {
            set_solana_config().map_err(|err| CliError::FailedLocalnet(err.to_string()))?
        }
        _ => {}
    }

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
        CliCommand::ConfigSet {
            admin,
            epoch_queue,
            hasher_queue,
        } => super::config::set(&client, admin, epoch_queue, hasher_queue),
        CliCommand::DelegationCreate { worker_id } => super::delegation::create(&client, worker_id),
        CliCommand::DelegationDeposit {
            amount,
            delegation_id,
            worker_id,
        } => super::delegation::deposit(&client, amount, delegation_id, worker_id),
        CliCommand::DelegationGet {
            delegation_id,
            worker_id,
        } => super::delegation::get(&client, delegation_id, worker_id),
        CliCommand::Initialize { mint } => super::initialize::initialize(&client, mint),
        CliCommand::Localnet { program_infos } => super::localnet::start(&client, program_infos),
        CliCommand::PoolGet { id } => super::pool::get(&client, id),
        CliCommand::PoolList {} => super::pool::list(&client),
        CliCommand::QueueCreate {
            id,
            kickoff_instruction,
            trigger,
        } => super::queue::create(&client, id, kickoff_instruction, trigger),
        CliCommand::QueueDelete { id } => super::queue::delete(&client, id),
        CliCommand::QueueGet { id } => super::queue::get(&client, id),
        CliCommand::QueueUpdate { id, rate_limit } => super::queue::update(&client, id, rate_limit),
        CliCommand::RegistryGet => super::registry::get(&client),
        CliCommand::WebhookRequestNew {
            api,
            id,
            method,
            route,
        } => super::webhook::request_new(&client, api, id, method, route),
        CliCommand::WorkerCreate { signatory } => super::worker::create(&client, signatory, false),
        CliCommand::WorkerGet { id } => super::worker::get(&client, id),
    }
}

fn set_solana_config() -> Result<()> {
    println!("Setting solana config");

    let mut process = std::process::Command::new("solana")
        .args(&["config", "set", "--url", "http://localhost:8899"])
        .spawn()
        .expect("failed to set solana config");

    process.wait()?;

    std::thread::sleep(std::time::Duration::from_secs(1));

    Ok(())
}
