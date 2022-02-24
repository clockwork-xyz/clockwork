use std::sync::Arc;

use anchor_lang::prelude::Pubkey;
use solana_client_helpers::Client;

use crate::{
    error::CliError,
    utils::sign_and_submit,
};

pub fn cancel(client: &Arc<Client>, address: &Pubkey) -> Result<(), CliError> {
    let owner = client.payer_pubkey();
    let daemon = cronos_sdk::account::Daemon::pda(owner).0;
    let ix = cronos_sdk::instruction::task_cancel(daemon, *address, owner);
    sign_and_submit(client, &[ix]);
    super::get(client, address)
}
