use std::sync::Arc;

use solana_client_helpers::Client;
use solana_sdk::pubkey::Pubkey;

use crate::{
    cli::CliError,
    utils::{sign_and_submit, solana_explorer_url, SolanaExplorerAccountType},
};

pub fn create(client: &Arc<Client>) -> Result<(), CliError> {
    let owner = client.payer_pubkey();
    let manager_pda = cronos_sdk::scheduler::state::Manager::pda(owner);
    let fee_pda = cronos_sdk::scheduler::state::Fee::pda(manager_pda.0);
    let ix =
        cronos_sdk::scheduler::instruction::manager_new(fee_pda.0, owner, owner, manager_pda.0);
    sign_and_submit(client, &[ix], &[client.payer()]);
    get(client, &manager_pda.0)
}

pub fn get(client: &Arc<Client>, address: &Pubkey) -> Result<(), CliError> {
    let data = client
        .get_account_data(&address)
        .map_err(|_err| CliError::AccountNotFound(address.to_string()))?;
    let manager_data = cronos_sdk::scheduler::state::Manager::try_from(data)
        .map_err(|_err| CliError::AccountDataNotParsable(address.to_string()))?;
    println!(
        "Explorer: {}",
        solana_explorer_url(SolanaExplorerAccountType::Account, address.to_string())
    );
    println!("{:#?}", manager_data);
    Ok(())
}
