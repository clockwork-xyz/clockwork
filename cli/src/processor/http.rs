use cronos_client::{http::state::HttpMethod, Client};
use solana_sdk::pubkey::Pubkey;

use crate::errors::CliError;

pub fn request_new(
    client: &Client,
    ack_authority: Pubkey,
    method: HttpMethod,
    url: String,
) -> Result<(), CliError> {
    // Get the request id
    let payer = client.payer_pubkey();
    let manager_pubkey = cronos_client::http::state::Manager::pubkey(payer);
    let id = client
        .get::<cronos_client::http::state::Manager>(&manager_pubkey)
        .map_or(0, |manager| manager.request_count);

    // Build the instruction
    let ix = cronos_client::http::instruction::request_new(ack_authority, id, method, payer, url);
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    Ok(())
}
