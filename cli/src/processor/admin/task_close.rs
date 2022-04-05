use std::sync::Arc;

use anchor_lang::prelude::Pubkey;
use solana_client_helpers::Client;

use crate::{error::CliError, utils::sign_and_submit};

pub fn task_close(client: &Arc<Client>, address: &Pubkey) -> Result<(), CliError> {
    // let admin = client.payer_pubkey();
    // let config = cronos_sdk::account::Config::pda().0;
    // let ix = cronos_sdk::instruction::admin_task_close(admin, config, *address);
    // sign_and_submit(client, &[ix]);
    Ok(())
}
