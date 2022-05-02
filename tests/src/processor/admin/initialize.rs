use std::sync::Arc;

use solana_client_helpers::Client;

use crate::{error::TestError, utils::sign_and_submit};

pub fn initialize(client: &Arc<Client>) -> Result<(), TestError> {
    let authority_pda = cronos_sdk::scheduler::state::Authority::pda();
    let config_pda = cronos_sdk::scheduler::state::Config::pda();
    let daemon_pda = cronos_sdk::scheduler::state::Daemon::pda(authority_pda.0);
    let fee_pda = cronos_sdk::scheduler::state::Fee::pda(daemon_pda.0);
    let ix = cronos_sdk::scheduler::instruction::admin_initialize(
        client.payer_pubkey(),
        authority_pda,
        config_pda,
        daemon_pda,
        fee_pda,
    );
    sign_and_submit(client, &[ix]);
    Ok(())
}
