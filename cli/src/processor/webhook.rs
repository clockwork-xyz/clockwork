use clockwork_client::{webhook::state::HttpMethod, Client};
use solana_sdk::pubkey::Pubkey;

use crate::errors::CliError;

pub fn request_new(
    client: &Client,
    api: Pubkey,
    id: String,
    method: HttpMethod,
    route: String,
) -> Result<(), CliError> {
    let ix = clockwork_client::webhook::instruction::request_new(
        api,
        client.payer_pubkey(),
        id,
        method,
        client.payer_pubkey(),
        route,
    );
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    Ok(())
}
