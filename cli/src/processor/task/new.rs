use std::sync::Arc;

use cronos_sdk::account::*;
use solana_client_helpers::Client;
use solana_sdk::instruction::Instruction;

use crate::{error::CliError, utils::sign_and_submit};

pub fn new(client: &Arc<Client>, ix: Instruction, schedule: String) -> Result<(), CliError> {
    // Fetch daemon data.
    let owner = client.payer_pubkey();
    let daemon_addr = Daemon::pda(owner).0;
    let data = client
        .get_account_data(&daemon_addr)
        .map_err(|_err| CliError::AccountNotFound(daemon_addr.to_string()))?;
    let daemon_data = Daemon::try_from(data)
        .map_err(|_err| CliError::AccountDataNotParsable(daemon_addr.to_string()))?;

    // Build task_create ix.
    let task_pda = Task::pda(daemon_addr, daemon_data.task_count);
    let config_addr = Config::pda().0;
    let task_ix = cronos_sdk::instruction::task_create(task_pda, daemon_addr, owner, ix, schedule);

    // Sign and submit
    sign_and_submit(client, &[task_ix]);
    super::get(client, &task_pda.0)
}
