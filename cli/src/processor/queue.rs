use {
    crate::errors::CliError,
    clockwork_client::{queue::objects::Queue, Client},
};

pub fn get(client: &Client, id: String) -> Result<(), CliError> {
    let queue_pubkey = Queue::pubkey(client.payer_pubkey(), id);
    let queue = client
        .get::<Queue>(&queue_pubkey)
        .map_err(|_err| CliError::AccountDataNotParsable(queue_pubkey.to_string()))?;

    println!("Address: {}\n{:#?}", queue_pubkey, queue);

    Ok(())
}

pub fn update(client: &Client, id: String, rate_limit: Option<u64>) -> Result<(), CliError> {
    let queue_pubkey = Queue::pubkey(client.payer_pubkey(), id);
    let ix = clockwork_client::queue::instruction::queue_update(
        client.payer_pubkey(),
        queue_pubkey,
        None,
        rate_limit,
        None,
    );
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    Ok(())
}
