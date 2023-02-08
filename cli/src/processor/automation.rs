use {
    crate::errors::CliError,
    clockwork_client::{
        automation::state::{Automation, AutomationSettings, Ix, Trigger},
        Client,
    },
    clockwork_utils::CrateInfo,
    solana_sdk::pubkey::Pubkey,
};

pub fn crate_info(client: &Client) -> Result<(), CliError> {
    let ix = clockwork_client::automation::instruction::get_crate_info();
    let crate_info: CrateInfo = client.get_return_data(ix).unwrap();
    println!("{:#?}", crate_info);
    Ok(())
}

pub fn create(
    client: &Client,
    id: String,
    instructions: Vec<Ix>,
    trigger: Trigger,
) -> Result<(), CliError> {
    let automation_pubkey = Automation::pubkey(client.payer_pubkey(), id.clone().into_bytes());
    let ix = clockwork_client::automation::instruction::automation_create(
        0,
        client.payer_pubkey(),
        id.into_bytes(),
        instructions,
        client.payer_pubkey(),
        automation_pubkey,
        trigger,
    );
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    get(client, automation_pubkey)?;
    Ok(())
}

pub fn delete(client: &Client, id: String) -> Result<(), CliError> {
    let automation_pubkey = Automation::pubkey(client.payer_pubkey(), id.into_bytes());
    let ix = clockwork_client::automation::instruction::automation_delete(
        client.payer_pubkey(),
        client.payer_pubkey(),
        automation_pubkey,
    );
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    Ok(())
}

pub fn get(client: &Client, address: Pubkey) -> Result<(), CliError> {
    let automation = client
        .get::<Automation>(&address)
        .map_err(|_err| CliError::AccountDataNotParsable(address.to_string()))?;
    println!("Address: {}\n{:#?}", address, automation);
    Ok(())
}

pub fn pause(client: &Client, id: String) -> Result<(), CliError> {
    let automation_pubkey = Automation::pubkey(client.payer_pubkey(), id.into_bytes());
    let ix = clockwork_client::automation::instruction::automation_pause(
        client.payer_pubkey(),
        automation_pubkey,
    );
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    get(client, automation_pubkey)?;
    Ok(())
}

pub fn resume(client: &Client, id: String) -> Result<(), CliError> {
    let automation_pubkey = Automation::pubkey(client.payer_pubkey(), id.into_bytes());
    let ix = clockwork_client::automation::instruction::automation_resume(
        client.payer_pubkey(),
        automation_pubkey,
    );
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    get(client, automation_pubkey)?;
    Ok(())
}

pub fn reset(client: &Client, id: String) -> Result<(), CliError> {
    let automation_pubkey = Automation::pubkey(client.payer_pubkey(), id.into_bytes());
    let ix = clockwork_client::automation::instruction::automation_reset(
        client.payer_pubkey(),
        automation_pubkey,
    );
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    get(client, automation_pubkey)?;
    Ok(())
}

pub fn update(
    client: &Client,
    id: String,
    rate_limit: Option<u64>,
    schedule: Option<String>,
) -> Result<(), CliError> {
    let automation_pubkey = Automation::pubkey(client.payer_pubkey(), id.into_bytes());
    let trigger = if let Some(schedule) = schedule {
        Some(Trigger::Cron {
            schedule,
            skippable: true,
        })
    } else {
        None
    };
    let settings = AutomationSettings {
        fee: None,
        instructions: None,
        name: None,
        rate_limit,
        trigger,
    };
    let ix = clockwork_client::automation::instruction::automation_update(
        client.payer_pubkey(),
        automation_pubkey,
        settings,
    );
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    get(client, automation_pubkey)?;
    Ok(())
}

pub fn parse_pubkey_from_id_or_address(
    authority: Pubkey,
    id: Option<String>,
    address: Option<Pubkey>,
) -> Result<Pubkey, CliError> {
    let address_from_id = id.map(|str| Automation::pubkey(authority, str.into()));
    address.or(address_from_id).ok_or(CliError::InvalidAddress)
}
