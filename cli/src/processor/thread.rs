use {
    crate::errors::CliError,
    clockwork_client::{
        thread::state::{InstructionData, Thread, ThreadSettings, Trigger},
        Client,
    },
    clockwork_utils::CrateInfo,
    solana_sdk::pubkey::Pubkey,
};

pub fn crate_info(client: &Client) -> Result<(), CliError> {
    let ix = clockwork_client::thread::instruction::get_crate_info();
    let crate_info: CrateInfo = client.get_return_data(ix).unwrap();
    println!("{:#?}", crate_info);
    Ok(())
}

pub fn create(
    client: &Client,
    id: String,
    kickoff_instruction: InstructionData,
    trigger: Trigger,
) -> Result<(), CliError> {
    let thread_pubkey = Thread::pubkey(client.payer_pubkey(), id.clone());
    let ix = clockwork_client::thread::instruction::thread_create(
        client.payer_pubkey(),
        id.clone(),
        kickoff_instruction,
        client.payer_pubkey(),
        thread_pubkey,
        trigger,
    );
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    get(client, thread_pubkey)?;
    Ok(())
}

pub fn delete(client: &Client, id: String) -> Result<(), CliError> {
    let thread_pubkey = Thread::pubkey(client.payer_pubkey(), id);
    let ix = clockwork_client::thread::instruction::thread_delete(
        client.payer_pubkey(),
        client.payer_pubkey(),
        thread_pubkey,
    );
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    Ok(())
}

pub fn get(client: &Client, address: Pubkey) -> Result<(), CliError> {
    let thread = client
        .get::<Thread>(&address)
        .map_err(|_err| CliError::AccountDataNotParsable(address.to_string()))?;
    println!("Address: {}\n{:#?}", address, thread);
    Ok(())
}

pub fn pause(client: &Client, id: String) -> Result<(), CliError> {
    let thread_pubkey = Thread::pubkey(client.payer_pubkey(), id.clone());
    let ix =
        clockwork_client::thread::instruction::thread_pause(client.payer_pubkey(), thread_pubkey);
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    get(client, thread_pubkey)?;
    Ok(())
}

pub fn resume(client: &Client, id: String) -> Result<(), CliError> {
    let thread_pubkey = Thread::pubkey(client.payer_pubkey(), id.clone());
    let ix =
        clockwork_client::thread::instruction::thread_resume(client.payer_pubkey(), thread_pubkey);
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    get(client, thread_pubkey)?;
    Ok(())
}

pub fn stop(client: &Client, id: String) -> Result<(), CliError> {
    let thread_pubkey = Thread::pubkey(client.payer_pubkey(), id.clone());
    let ix =
        clockwork_client::thread::instruction::thread_stop(client.payer_pubkey(), thread_pubkey);
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    get(client, thread_pubkey)?;
    Ok(())
}

pub fn update(
    client: &Client,
    id: String,
    rate_limit: Option<u64>,
    schedule: Option<String>,
) -> Result<(), CliError> {
    let thread_pubkey = Thread::pubkey(client.payer_pubkey(), id.clone());
    let trigger = if let Some(schedule) = schedule {
        Some(Trigger::Cron {
            schedule,
            skippable: true,
        })
    } else {
        None
    };
    let settings = ThreadSettings {
        fee: None,
        kickoff_instruction: None,
        rate_limit,
        trigger,
    };
    let ix = clockwork_client::thread::instruction::thread_update(
        client.payer_pubkey(),
        thread_pubkey,
        settings,
    );
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    get(client, thread_pubkey)?;
    Ok(())
}

pub fn parse_pubkey_from_id_or_address(
    authority: Pubkey,
    id: Option<String>,
    address: Option<Pubkey>,
) -> Result<Pubkey, CliError> {
    let address_from_id = id.map(|str| Thread::pubkey(authority, str.into()));

    address.or(address_from_id).ok_or(CliError::InvalidAddress)
}
