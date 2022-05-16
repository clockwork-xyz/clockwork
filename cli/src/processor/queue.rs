use {
    crate::{
        cli::CliError,
        utils::{sign_and_submit, solana_explorer_url, SolanaExplorerAccountType},
    },
    cronos_sdk::scheduler::state::{Queue, Yogi},
    solana_client_helpers::Client,
    solana_sdk::pubkey::Pubkey,
    std::sync::Arc,
};

pub fn cancel(client: &Arc<Client>, address: &Pubkey) -> Result<(), CliError> {
    let owner = client.payer_pubkey();
    let yogi = cronos_sdk::scheduler::state::Yogi::pda(owner).0;
    let ix = cronos_sdk::scheduler::instruction::queue_cancel(yogi, *address, owner);
    sign_and_submit(client, &[ix], &[client.payer()]);
    get(client, address)
}

pub fn create(client: &Arc<Client>, schedule: String) -> Result<(), CliError> {
    // Fetch yogi data.
    let owner = client.payer_pubkey();
    let yogi_addr = Yogi::pda(owner).0;
    let data = client
        .get_account_data(&yogi_addr)
        .map_err(|_err| CliError::AccountNotFound(yogi_addr.to_string()))?;
    let yogi_data = Yogi::try_from(data)
        .map_err(|_err| CliError::AccountDataNotParsable(yogi_addr.to_string()))?;

    // Build queue_create ix.
    let queue_pubkey = Queue::pda(yogi_addr, yogi_data.queue_count).0;
    let queue_ix = cronos_sdk::scheduler::instruction::queue_new(
        owner,
        owner,
        yogi_addr,
        schedule,
        queue_pubkey,
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
