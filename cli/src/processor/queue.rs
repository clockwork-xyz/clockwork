use {
    crate::errors::CliError,
    cronos_client::{
        scheduler::state::{Delegate, Queue},
        Client,
    },
    solana_sdk::pubkey::Pubkey,
};

pub fn create(client: &Client, schedule: String) -> Result<(), CliError> {
    // Fetch delegate data.
    let authority_pubkey = client.payer_pubkey();
    let delegate_pubkey = Delegate::pubkey(authority_pubkey);
    let data = client
        .get_account_data(&delegate_pubkey)
        .map_err(|_err| CliError::AccountNotFound(delegate_pubkey.to_string()))?;
    let delegate_data = Delegate::try_from(data)
        .map_err(|_err| CliError::AccountDataNotParsable(delegate_pubkey.to_string()))?;

    // Build queue_create ix.
    let queue_pubkey = Queue::pubkey(delegate_pubkey, delegate_data.queue_count);
    let queue_ix = cronos_client::scheduler::instruction::queue_new(
        authority_pubkey,
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
        let task_pubkey = cronos_client::scheduler::state::Task::pubkey(*address, i);
        task_pubkeys.push(task_pubkey);
    }

    println!("Tasks: {:#?}", task_pubkeys);

    match task_id {
        None => (),
        Some(task_id) => {
            let task_pubkey = cronos_client::scheduler::state::Task::pubkey(*address, task_id);
            super::task::get(client, &task_pubkey).ok();
        }
    }

    Ok(())
}
