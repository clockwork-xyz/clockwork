use {
    crate::{error::CliError, utils::sign_and_submit},
    cronos_sdk::scheduler::state::*,
    solana_client_helpers::Client,
    solana_sdk::instruction::Instruction,
    std::sync::Arc,
};

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
    let task_ix = cronos_sdk::scheduler::instruction::task_new(
        task_pda,
        daemon_addr,
        owner,
        vec![ix],
        schedule,
    );

    // Sign and submit
    sign_and_submit(client, &[task_ix]);
    super::get(client, &task_pda.0)
}
