use std::sync::Arc;

use solana_client_helpers::Client;

use crate::{error::CliError, utils::sign_and_submit};

pub fn initialize(client: &Arc<Client>) -> Result<(), CliError> {
    let authority_pda = cronos_sdk::account::Authority::pda();
    let config_pda = cronos_sdk::account::Config::pda();
    let daemon_pda = cronos_sdk::account::Daemon::pda(authority_pda.0);
    let fee_pda = cronos_sdk::account::Fee::pda(daemon_pda.0);
    let health_pda = cronos_sdk::account::Health::pda();
    let treasury_pda = cronos_sdk::account::Treasury::pda();
    let ix = cronos_sdk::instruction::admin_initialize(
        authority_pda,
        config_pda,
        daemon_pda,
        fee_pda,
        health_pda,
        treasury_pda,
        client.payer_pubkey(),
    );
    sign_and_submit(client, &[ix]);
    Ok(())
}
