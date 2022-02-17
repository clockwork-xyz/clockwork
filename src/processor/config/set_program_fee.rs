use std::sync::Arc;

use solana_client_helpers::Client;

use crate::{error::CliError, utils::sign_and_submit};

pub fn set_program_fee(client: &Arc<Client>, new_program_fee: &u64) -> Result<(), CliError> {
    let admin = client.payer_pubkey();
    let config = cronos_sdk::account::Config::pda().0;
    let ix = cronos_sdk::instruction::admin_update_program_fee(admin, config, *new_program_fee);
    sign_and_submit(client, &[ix]);
    super::get(client)
}
