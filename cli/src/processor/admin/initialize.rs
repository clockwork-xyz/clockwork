use std::sync::Arc;

use solana_client_helpers::Client;

use crate::{error::CliError, utils::sign_and_submit};

pub fn initialize(client: &Arc<Client>) -> Result<(), CliError> {
    let authority_pda = cronos_sdk::cronos::state::Authority::pda();
    let config_pda = cronos_sdk::cronos::state::Config::pda();
    let daemon_pda = cronos_sdk::cronos::state::Daemon::pda(authority_pda.0);
    let fee_pda = cronos_sdk::cronos::state::Fee::pda(daemon_pda.0);
    let ix = cronos_sdk::cronos::instruction::admin_initialize(
        client.payer_pubkey(),
        authority_pda,
        config_pda,
        daemon_pda,
        fee_pda,
    );
    sign_and_submit(client, &[ix]);
    Ok(())
}
