use {
    crate::cli::CliError,
    cronos_client::{
        scheduler::state::{Manager, Queue},
        Client,
    },
    solana_sdk::pubkey::Pubkey,
};

pub fn create(client: &Client, schedule: String) -> Result<(), CliError> {
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
    let queue_ix = cronos_client::scheduler::instruction::queue_new(
        authority_pubkey,
        manager_pubkey,
        authority_pubkey,
        queue_pubkey,
        schedule,
    );

    // Sign and submit
    client
        .send_and_confirm(&[queue_ix], &[client.payer()])
        .unwrap();
    get(client, &queue_pubkey)
}

pub fn get(client: &Client, address: &Pubkey) -> Result<(), CliError> {
    let queue = client
        .get::<Queue>(&address)
        .map_err(|_err| CliError::AccountDataNotParsable(address.to_string()))?;
    println!("{:#?}", queue);
    Ok(())
}
