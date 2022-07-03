use cronos_client::{http::state::HttpMethod, Client};

use crate::errors::CliError;

pub fn request_new(client: &Client, method: HttpMethod, url: String) -> Result<(), CliError> {
    let payer = client.payer_pubkey();
    let ix = cronos_client::http::instruction::request_new(method, payer, url);
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    Ok(())
}
