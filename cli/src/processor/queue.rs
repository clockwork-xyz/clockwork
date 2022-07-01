use {
    crate::errors::CliError,
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
    get(client, &queue_pubkey, None)
}

pub fn get(client: &Client, address: &Pubkey, task_id: Option<u128>) -> Result<(), CliError> {
    let queue = client
        .get::<Queue>(&address)
        .map_err(|_err| CliError::AccountDataNotParsable(address.to_string()))?;
    println!("{:#?}", queue);

    let mut task_pubkeys = vec![];
    for i in 0..queue.task_count.min(10) {
        let task_pubkey = cronos_client::scheduler::state::Task::pda(*address, i).0;
        task_pubkeys.push(task_pubkey);
    }

    println!("Tasks: {:#?}", task_pubkeys);

    match task_id {
        None => (),
        Some(task_id) => {
            let task_pubkey = cronos_client::scheduler::state::Task::pda(*address, task_id).0;
            super::task::get(client, &task_pubkey).ok();
        }
    }

    Ok(())
}
