// use anchor_lang::prelude::*;
use anchor_lang::{InstructionData, ToAccountMetas};
use clockwork_client::{
    webhook::state::{HttpMethod, Webhook},
    Client,
};
use solana_sdk::{instruction::Instruction, system_program};

use crate::errors::CliError;

pub fn create(
    client: &Client,
    id: Vec<u8>,
    method: HttpMethod,
    url: String,
) -> Result<(), CliError> {
    let ix = Instruction {
        program_id: clockwork_webhook_program::ID,
        accounts: clockwork_webhook_program::accounts::WebhookCreate {
            authority: client.payer_pubkey(),
            payer: client.payer_pubkey(),
            webhook: Webhook::pubkey(client.payer_pubkey(), id.clone()),
            system_program: system_program::ID,
        }
        .to_account_metas(Some(true)),
        data: clockwork_webhook_program::instruction::WebhookCreate {
            id: id.clone(),
            method,
            url,
        }
        .data(),
    };
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    get(client, id)?;

    Ok(())
}

pub fn get(client: &Client, id: Vec<u8>) -> Result<(), CliError> {
    let address = Webhook::pubkey(client.payer_pubkey(), id.clone());
    let webhook = client
        .get::<Webhook>(&address)
        .map_err(|_err| CliError::AccountDataNotParsable(address.to_string()))?;
    println!("Address: {}\n{:#?}", address, webhook);
    todo!()
}
