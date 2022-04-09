use {
    crate::{error::CliError, utils::sign_and_submit},
    anchor_lang::prelude::Pubkey,
    solana_client_helpers::Client,
    std::sync::Arc,
};

pub fn cancel(client: &Arc<Client>, address: &Pubkey) -> Result<(), CliError> {
    let owner = client.payer_pubkey();
    let daemon = cronos_sdk::cronos::state::Daemon::pda(owner).0;
    let ix = cronos_sdk::cronos::instruction::task_cancel(daemon, *address, owner);
    sign_and_submit(client, &[ix]);
    super::get(client, address)
}
