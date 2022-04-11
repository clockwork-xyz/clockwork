use std::sync::Arc;

use solana_sdk::pubkey::Pubkey;
use solana_client_helpers::Client;

use crate::{error::CliError, utils::sign_and_submit};

pub fn task_cancel(client: &Arc<Client>, address: &Pubkey) -> Result<(), CliError> {
    let admin = client.payer_pubkey();
    let config = cronos_sdk::scheduler::state::Config::pda().0;
    let ix = cronos_sdk::scheduler::instruction::admin_task_cancel(admin, config, *address);
    sign_and_submit(client, &[ix]);
    Ok(())
}
