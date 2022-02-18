use std::sync::Arc;

use solana_client_helpers::Client;

use crate::{error::CliError, utils::sign_and_submit};

pub fn health_reset(client: &Arc<Client>) -> Result<(), CliError> {
    let admin = client.payer_pubkey();
    let config = cronos_sdk::account::Config::pda().0;
    let health = cronos_sdk::account::Health::pda().0;
    let ix = cronos_sdk::instruction::admin_reset_health(admin, config, health);
    sign_and_submit(client, &[ix]);
    Ok(())
}
