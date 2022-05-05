use cronos_sdk::scheduler::state::{Daemon, Task};
use solana_sdk::instruction::Instruction;

use crate::utils::{solana_explorer_url, SolanaExplorerAccountType};

use {
    crate::{cli::CliError, utils::sign_and_submit},
    solana_client_helpers::Client,
    solana_sdk::pubkey::Pubkey,
    std::sync::Arc,
};

pub fn cancel(client: &Arc<Client>, address: &Pubkey) -> Result<(), CliError> {
    let owner = client.payer_pubkey();
    let daemon = cronos_sdk::scheduler::state::Daemon::pda(owner).0;
    let ix = cronos_sdk::scheduler::instruction::task_cancel(daemon, *address, owner);
    sign_and_submit(client, &[ix]);
    get(client, address)
}

pub fn create(client: &Arc<Client>, ix: Instruction, schedule: String) -> Result<(), CliError> {
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
    let task_ix = cronos_sdk::scheduler::instruction::task_new(
        task_pda,
        daemon_addr,
        owner,
        vec![ix],
        schedule,
    );

    // Sign and submit
    sign_and_submit(client, &[task_ix]);
    get(client, &task_pda.0)
}

pub fn get(client: &Arc<Client>, address: &Pubkey) -> Result<(), CliError> {
    let data = client
        .get_account_data(address)
        .map_err(|_err| CliError::AccountNotFound(address.to_string()))?;
    let task_data = Task::try_from(data)
        .map_err(|_err| CliError::AccountDataNotParsable(address.to_string()))?;
    println!(
        "Explorer: {}",
        solana_explorer_url(SolanaExplorerAccountType::Account, address.to_string())
    );
    println!("{:#?}", task_data);
    Ok(())
}
