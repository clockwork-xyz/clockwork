use {
    crate::{error::CliError, utils::sign_and_submit},
    solana_client_helpers::Client,
    std::sync::Arc
};

pub fn open(client: &Arc<Client>) -> Result<(), CliError> {
    let owner = client.payer_pubkey();
    let daemon_pda = cronos_sdk::account::Daemon::pda(owner);
    let fee_pda = cronos_sdk::account::Fee::pda(daemon_pda.0);
    let ix = cronos_sdk::instruction::daemon_open(daemon_pda, fee_pda, owner);
    sign_and_submit(client, &[ix]);
    super::get(client)
}
