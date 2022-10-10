use crate::errors::CliError;
use clockwork_client::Client;
use solana_sdk::pubkey::Pubkey;

pub fn api_new(
    _client: &Client,
    _ack_authority: Pubkey,
    _base_url: String,
) -> Result<(), CliError> {
    // TODO Come back to this when we do webhooks!
    //
    // let authority_pubkey = client.payer_pubkey();
    // let api_pubkey =
    //     clockwork_client::webhook::objects::Api::pubkey(authority_pubkey, base_url.clone());
    // let ix = clockwork_client::webhook::instruction::api_new(
    //     ack_authority,
    //     authority_pubkey,
    //     base_url,
    //     authority_pubkey,
    // );
    // client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    // println!("New api created: {}", api_pubkey);
    Ok(())
}
