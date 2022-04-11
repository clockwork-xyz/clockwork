use {
    crate::{error::CliError, utils::sign_and_submit},
    solana_client_helpers::Client,
    solana_sdk::pubkey::Pubkey,
    std::sync::Arc,
};

pub fn cancel(client: &Arc<Client>, address: &Pubkey) -> Result<(), CliError> {
    let owner = client.payer_pubkey();
    let daemon = cronos_sdk::scheduler::state::Daemon::pda(owner).0;
    let ix = cronos_sdk::scheduler::instruction::task_cancel(daemon, *address, owner);
    sign_and_submit(client, &[ix]);
    super::get(client, address)
}
