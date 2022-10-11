use {
    crate::errors::CliError,
    clockwork_client::{
        queue::objects::{Queue, QueueSettings, Trigger},
        Client,
    },
    clockwork_utils::InstructionData,
};

pub fn create(
    client: &Client,
    id: String,
    kickoff_instruction: InstructionData,
    trigger: Trigger,
) -> Result<(), CliError> {
    let queue_pubkey = Queue::pubkey(client.payer_pubkey(), id.clone());
    let ix = clockwork_client::queue::instruction::queue_create(
        client.payer_pubkey(),
        id.clone(),
        kickoff_instruction,
        client.payer_pubkey(),
        queue_pubkey,
        trigger,
    );
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    get(client, id)?;
    Ok(())
}

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
    let settings = QueueSettings {
        fee: None,
        kickoff_instruction: None,
        rate_limit,
        trigger: None,
    };
    let ix = clockwork_client::queue::instruction::queue_update(
        client.payer_pubkey(),
        queue_pubkey,
        settings,
    );
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    Ok(())
}
