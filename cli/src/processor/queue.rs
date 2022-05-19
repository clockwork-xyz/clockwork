use {
    crate::{
        cli::CliError,
        utils::{sign_and_submit, solana_explorer_url, SolanaExplorerAccountType},
    },
    cronos_sdk::scheduler::state::{Manager, Queue},
    solana_client_helpers::Client,
    solana_sdk::pubkey::Pubkey,
    std::sync::Arc,
};

pub fn create(client: &Arc<Client>, schedule: String) -> Result<(), CliError> {
    // Fetch manager data.
    let authority_pubkey = client.payer_pubkey();
    let manager_pubkey = Manager::pda(authority_pubkey).0;
    let data = client
        .get_account_data(&manager_pubkey)
        .map_err(|_err| CliError::AccountNotFound(manager_pubkey.to_string()))?;
    let manager_data = Manager::try_from(data)
        .map_err(|_err| CliError::AccountDataNotParsable(manager_pubkey.to_string()))?;

    // Build queue_create ix.
    let queue_pubkey = Queue::pda(manager_pubkey, manager_data.queue_count).0;
    let queue_ix = cronos_sdk::scheduler::instruction::queue_new(
        authority_pubkey,
        manager_pubkey,
        authority_pubkey,
        queue_pubkey,
        schedule,
    );

    // Sign and submit
    sign_and_submit(client, &[queue_ix], &[client.payer()]);
    get(client, &queue_pubkey)
}

pub fn get(client: &Arc<Client>, address: &Pubkey) -> Result<(), CliError> {
    let data = client
        .get_account_data(address)
        .map_err(|_err| CliError::AccountNotFound(address.to_string()))?;
    let queue_data = Queue::try_from(data)
        .map_err(|_err| CliError::AccountDataNotParsable(address.to_string()))?;
    println!(
        "Explorer: {}",
        solana_explorer_url(SolanaExplorerAccountType::Account, address.to_string())
    );
    println!("{:#?}", queue_data);
    Ok(())
}
