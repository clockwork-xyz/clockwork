use anchor_lang::{
    solana_program::{instruction::{AccountMeta, Instruction}, system_program},
    InstructionData, AccountDeserialize
};
use clockwork_thread_program::state::{SerializableInstruction, Thread, ThreadSettings, Trigger, VersionedThread};
use clockwork_utils::CrateInfo;
use solana_sdk::pubkey::Pubkey;

use crate::{client::Client, errors::CliError};

pub fn crate_info(client: &Client) -> Result<(), CliError> {
    // let ix = clockwork_client::thread::instruction::get_crate_info();
    let ix = Instruction {
        program_id: clockwork_thread_program::ID,
        accounts: vec![AccountMeta::new_readonly(system_program::ID, false)],
        data: clockwork_thread_program::instruction::GetCrateInfo {}.data(),
    };
    let crate_info: CrateInfo = client.get_return_data(ix).unwrap();
    println!("{:#?}", crate_info);
    Ok(())
}

pub fn create(
    client: &Client,
    id: String,
    instructions: Vec<SerializableInstruction>,
    trigger: Trigger,
) -> Result<(), CliError> {
    let thread_pubkey = Thread::pubkey(client.payer_pubkey(), id.clone().into_bytes());
    // let ix = clockwork_client::thread::instruction::thread_create(
    //     0,
    //     client.payer_pubkey(),
    //     id.into_bytes(),
    //     instructions,
    //     client.payer_pubkey(),
    //     thread_pubkey,
    //     trigger,
    // );
    let ix = Instruction {
        program_id: clockwork_thread_program::ID,
        accounts: vec![
            AccountMeta::new_readonly(client.payer_pubkey(), true),
            AccountMeta::new(client.payer_pubkey(), true),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new(thread_pubkey, false),
        ],
        data: clockwork_thread_program::instruction::ThreadCreate {
            amount: 0,
            id: id.into_bytes(),
            instructions,
            trigger,
        }
        .data(),
    };
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    get(client, thread_pubkey)?;
    Ok(())
}

pub fn delete(client: &Client, id: String) -> Result<(), CliError> {
    let thread_pubkey = Thread::pubkey(client.payer_pubkey(), id.into_bytes());
    // let ix = clockwork_client::thread::instruction::thread_delete(
    //     client.payer_pubkey(),
    //     client.payer_pubkey(),
    //     thread_pubkey,
    // );
    let ix = Instruction {
        program_id: clockwork_thread_program::ID,
        accounts: vec![
            AccountMeta::new_readonly(client.payer_pubkey(), true),
            AccountMeta::new(client.payer_pubkey(), true),
            AccountMeta::new(thread_pubkey, false),
        ],
        data: clockwork_thread_program::instruction::ThreadDelete {}.data(),
    };
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    Ok(())
}

pub fn get(client: &Client, address: Pubkey) -> Result<(), CliError> {
    let data = client.get_account_data(&address).unwrap();
    let thread = VersionedThread::try_deserialize(&mut data.as_slice()).unwrap();
    // let thread = client
    //     .get::<Thread>(&address)
    //     .map_err(|_err| CliError::AccountDataNotParsable(address.to_string()))?;
    println!("Address: {}\n{:#?}", address, thread);
    Ok(())
}

pub fn pause(client: &Client, id: String) -> Result<(), CliError> {
    let thread_pubkey = Thread::pubkey(client.payer_pubkey(), id.into_bytes());
    let ix = Instruction {
        program_id: clockwork_thread_program::ID,
        accounts: vec![
            AccountMeta::new_readonly(client.payer_pubkey(), true),
            AccountMeta::new(thread_pubkey, false),
        ],
        data: clockwork_thread_program::instruction::ThreadPause {}.data(),
    };
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    get(client, thread_pubkey)?;
    Ok(())
}

pub fn resume(client: &Client, id: String) -> Result<(), CliError> {
    let thread_pubkey = Thread::pubkey(client.payer_pubkey(), id.into_bytes());
    // let ix =
    //     clockwork_client::thread::instruction::thread_resume(client.payer_pubkey(), thread_pubkey);
    let ix = Instruction {
        program_id: clockwork_thread_program::ID,
        accounts: vec![
            AccountMeta::new_readonly(client.payer_pubkey(), true),
            AccountMeta::new(thread_pubkey, false),
        ],
        data: clockwork_thread_program::instruction::ThreadResume {}.data(),
    };
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    get(client, thread_pubkey)?;
    Ok(())
}

pub fn reset(client: &Client, id: String) -> Result<(), CliError> {
    let thread_pubkey = Thread::pubkey(client.payer_pubkey(), id.into_bytes());
    // let ix =
    //     clockwork_client::thread::instruction::thread_reset(client.payer_pubkey(), thread_pubkey);
    let ix = Instruction {
        program_id: clockwork_thread_program::ID,
        accounts: vec![
            AccountMeta::new_readonly(client.payer_pubkey(), true),
            AccountMeta::new(thread_pubkey, false),
        ],
        data: clockwork_thread_program::instruction::ThreadReset {}.data(),
    };
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
    let thread_pubkey = Thread::pubkey(client.payer_pubkey(), id.into_bytes());
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
        instructions: None,
        name: None,
        rate_limit,
        trigger,
    };
    // let ix = clockwork_client::thread::instruction::thread_update(
    //     client.payer_pubkey(),
    //     thread_pubkey,
    //     settings,
    // );
    let ix = Instruction {
        program_id: clockwork_thread_program::ID,
        accounts: vec![
            AccountMeta::new_readonly(client.payer_pubkey(), true),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new(thread_pubkey, false),
        ],
        data: clockwork_thread_program::instruction::ThreadUpdate { settings }.data(),
    };
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
