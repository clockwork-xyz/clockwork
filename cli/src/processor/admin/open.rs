use std::sync::Arc;

use solana_client_helpers::Client;

use crate::{error::CliError, utils::sign_and_submit};

pub fn open(client: &Arc<Client>) -> Result<(), CliError> {
    let authority_pda = cronos_sdk::account::Authority::pda();
    let config_pda = cronos_sdk::account::Config::pda();
    let daemon_pda = cronos_sdk::account::Daemon::pda(authority_pda.0);
    let fee_pda = cronos_sdk::account::Fee::pda(daemon_pda.0);
    let health_pda = cronos_sdk::account::Health::pda();
    let ix = cronos_sdk::instruction::admin_open(
        client.payer_pubkey(),
        authority_pda,
        config_pda,
        daemon_pda,
        fee_pda,
        health_pda,  
    );
    sign_and_submit(client, &[ix]);
    Ok(())
}
