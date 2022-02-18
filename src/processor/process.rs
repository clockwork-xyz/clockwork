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
        CliCommand::AdminCancelTask { address } => super::admin::cancel_task(&client, &address),
        CliCommand::AdminScheduleHealthCheck => super::admin::schedule_health_check(&client),
        CliCommand::Blocktime => super::blocktime::get(&client),
        CliCommand::ConfigGet => super::config::get(&client),
        CliCommand::ConfigSetMinRecurr { new_value } => {
            super::config::set_min_recurr(&client, &new_value)
        }
        CliCommand::ConfigSetProgramFee { new_value } => {
            super::config::set_program_fee(&client, &new_value)
        }
        CliCommand::ConfigSetWorkerFee { new_value } => {
            super::config::set_worker_fee(&client, &new_value)
        }
        CliCommand::DaemonNew => super::daemon::new(&client),
        CliCommand::DaemonGet => super::daemon::get(&client),
        CliCommand::HealthGet => super::health::get(&client),
        CliCommand::TaskCancel { address } => super::task::cancel(&client, &address),
        CliCommand::TaskGet { address } => super::task::get(&client, &address),
        CliCommand::TaskNew {
            ix,
            exec_at,
            stop_at,
            recurr,
        } => super::task::new(&client, ix, exec_at, stop_at, recurr),
    }
}
