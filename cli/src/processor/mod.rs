mod config;
mod crontab;
mod delegation;
mod explorer;
mod initialize;
mod localnet;
mod pool;
mod registry;
mod secret;
mod thread;
mod webhook;
mod worker;

use anyhow::Result;
use clap::ArgMatches;
use solana_sdk::signature::read_keypair_file;

use crate::{
    client::Client,
    cli::CliCommand,
    config::CliConfig,
    errors::CliError,
    processor::thread::parse_pubkey_from_id_or_address,
};

pub fn process(matches: &ArgMatches) -> Result<(), CliError> {
    // Parse command and config
    let command = CliCommand::try_from(matches)?;

    match command {
        // Set solana config if using localnet command
        CliCommand::Localnet { .. } => {
            // TODO Verify the Solana CLI version is compatable with this build.
            set_solana_config().map_err(|err| CliError::FailedLocalnet(err.to_string()))?
        }
        _ => {}
    }

    let mut config = CliConfig::load();

    // Build the RPC client
    let payer = read_keypair_file(&config.keypair_path)
        .map_err(|_| CliError::KeypairNotFound(config.keypair_path.clone()))?;

    let client = Client::new(payer, config.json_rpc_url.clone());

    // Process the command
    match command {
        CliCommand::ConfigGet => config::get(&client),
        CliCommand::ConfigSet {
            admin,
            epoch_thread,
            hasher_thread,
        } => config::set(&client, admin, epoch_thread, hasher_thread),
        CliCommand::Crontab { schedule } => crontab::get(&client, schedule),
        CliCommand::DelegationCreate { worker_id } => delegation::create(&client, worker_id),
        CliCommand::DelegationDeposit {
            amount,
            delegation_id,
            worker_id,
        } => delegation::deposit(&client, amount, delegation_id, worker_id),
        CliCommand::DelegationGet {
            delegation_id,
            worker_id,
        } => delegation::get(&client, delegation_id, worker_id),
        CliCommand::DelegationWithdraw {
            amount,
            delegation_id,
            worker_id,
        } => delegation::withdraw(&client, amount, delegation_id, worker_id),
        CliCommand::ExplorerGetThread { id, address } => {
            let pubkey = parse_pubkey_from_id_or_address(client.payer_pubkey(), id, address)?;
            explorer::thread_url(pubkey, config)
        }
        CliCommand::Initialize { mint } => initialize::initialize(&client, mint),
        CliCommand::Localnet {
            clone_addresses,
            network_url,
            program_infos,
            force_init,
            solana_archive,
            clockwork_archive,
            dev,
        } => localnet::start(
            &mut config,
            &client,
            clone_addresses,
            network_url,
            program_infos,
            force_init,
            solana_archive,
            clockwork_archive,
            dev,
        ),
        CliCommand::PoolGet { id } => pool::get(&client, id),
        CliCommand::PoolList {} => pool::list(&client),
        CliCommand::PoolUpdate { id, size } => pool::update(&client, id, size),
        CliCommand::SecretApprove { name, delegate } => secret::approve(&client, name, delegate),
        CliCommand::SecretRevoke { name, delegate } => secret::revoke(&client, name, delegate),
        CliCommand::SecretCreate { name, word } => secret::create(&client, name, word),
        CliCommand::SecretGet { name } => secret::get(&client, name),
        CliCommand::SecretList {} => secret::list(&client),
        CliCommand::ThreadCrateInfo {} => thread::crate_info(&client),
        CliCommand::ThreadCreate {
            id,
            kickoff_instruction,
            trigger,
        } => thread::create(&client, id, vec![kickoff_instruction], trigger),
        CliCommand::ThreadDelete { id } => thread::delete(&client, id),
        CliCommand::ThreadPause { id } => thread::pause(&client, id),
        CliCommand::ThreadResume { id } => thread::resume(&client, id),
        CliCommand::ThreadReset { id } => thread::reset(&client, id),
        CliCommand::ThreadGet { id, address } => {
            let pubkey = parse_pubkey_from_id_or_address(client.payer_pubkey(), id, address)?;
            thread::get(&client, pubkey)
        }
        CliCommand::ThreadUpdate {
            id,
            rate_limit,
            schedule,
        } => thread::update(&client, id, rate_limit, schedule),
        CliCommand::RegistryGet => registry::get(&client),
        CliCommand::RegistryUnlock => registry::unlock(&client),
        CliCommand::WebhookCreate {
            body,
            id,
            method,
            url,
        } => webhook::create(&client, body, id, method, url),
        CliCommand::WebhookGet { id } => webhook::get(&client, id),
        CliCommand::WorkerCreate { signatory } => worker::create(&client, signatory, false),
        CliCommand::WorkerGet { id } => worker::get(&client, id),
        CliCommand::WorkerUpdate { id, signatory } => worker::update(&client, id, signatory),
    }
}

fn set_solana_config() -> Result<()> {
    let mut process = std::process::Command::new("solana")
        .args(&["config", "set", "--url", "l"])
        .spawn()
        .expect("Failed to set solana config");
    process.wait()?;
    std::thread::sleep(std::time::Duration::from_secs(1));
    Ok(())
}
