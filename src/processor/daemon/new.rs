use std::sync::Arc;

use solana_client_helpers::Client;

use crate::{error::CliError, utils::sign_and_submit};

pub fn new(client: &Arc<Client>) -> Result<(), CliError> {
    let owner = client.payer_pubkey();
    let daemon_pda = cronos_sdk::account::Daemon::find_pda(owner);
    let fee_pda = cronos_sdk::account::Fee::find_pda(daemon_pda.0);
    let ix = cronos_sdk::instruction::daemon_create(daemon_pda, fee_pda, owner);
    sign_and_submit(client, &[ix]);
    Ok(())
}
