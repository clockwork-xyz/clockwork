use std::sync::Arc;

use solana_client_helpers::Client;

use crate::{error::CliError, utils::sign_and_submit};

pub fn set_min_recurr(client: &Arc<Client>, new_min_recurr: &i64) -> Result<(), CliError> {
    let admin = client.payer_pubkey();
    let config = cronos_sdk::account::Config::find_pda().0;
    let ix = cronos_sdk::instruction::admin_update_min_recurr(admin, config, *new_min_recurr);
    sign_and_submit(client, &[ix]);
    super::get(client)
}
