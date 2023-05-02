use anchor_lang::{
    solana_program::{instruction::Instruction, system_program},
    InstructionData, AccountDeserialize, ToAccountMetas
};
use clockwork_thread_program::state::{SerializableInstruction, Thread, ThreadSettings, Trigger, VersionedThread};
use clockwork_utils::CrateInfo;
use solana_sdk::pubkey::Pubkey;

use crate::{client::Client, errors::CliError};

pub fn crate_info(client: &Client) -> Result<(), CliError> {
    let ix = Instruction {
        program_id: clockwork_thread_program::ID,
        accounts: clockwork_thread_program::accounts::GetCrateInfo {
            system_program: system_program::ID,
        }.to_account_metas(Some(false)),
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
    let ix = Instruction {
        program_id: clockwork_thread_program::ID,
        accounts: clockwork_thread_program::accounts::ThreadCreate {
            authority: client.payer_pubkey(),
            payer: client.payer_pubkey(),
            system_program: system_program::ID,
            thread: thread_pubkey
        }.to_account_metas(Some(false)),
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
    let ix = Instruction {
        program_id: clockwork_thread_program::ID,
        accounts: clockwork_thread_program::accounts::ThreadDelete {
            authority: client.payer_pubkey(),
            close_to: client.payer_pubkey(),
            thread: thread_pubkey,
        }.to_account_metas(Some(false)),
        data: clockwork_thread_program::instruction::ThreadDelete {}.data(),
    };
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    Ok(())
}

pub fn get(client: &Client, address: Pubkey) -> Result<(), CliError> {
    let data = client.get_account_data(&address).unwrap();
    let thread = VersionedThread::try_deserialize(&mut data.as_slice()).unwrap();
    println!("Address: {}\n{:#?}", address, thread);
    Ok(())
}

pub fn pause(client: &Client, id: String) -> Result<(), CliError> {
    let thread_pubkey = Thread::pubkey(client.payer_pubkey(), id.into_bytes());
    let ix = Instruction {
        program_id: clockwork_thread_program::ID,
        accounts: clockwork_thread_program::accounts::ThreadPause {
            authority: client.payer_pubkey(),
            thread: thread_pubkey,
        }.to_account_metas(Some(false)),
        data: clockwork_thread_program::instruction::ThreadPause {}.data(),
    };
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    get(client, thread_pubkey)?;
    Ok(())
}

pub fn resume(client: &Client, id: String) -> Result<(), CliError> {
    let thread_pubkey = Thread::pubkey(client.payer_pubkey(), id.into_bytes());
    let ix = Instruction {
        program_id: clockwork_thread_program::ID,
        accounts: clockwork_thread_program::accounts::ThreadResume {
            authority: client.payer_pubkey(),
            thread: thread_pubkey
        }.to_account_metas(Some(false)),
        data: clockwork_thread_program::instruction::ThreadResume {}.data(),
    };
    client.send_and_confirm(&[ix], &[client.payer()]).unwrap();
    get(client, thread_pubkey)?;
    Ok(())
}

pub fn reset(client: &Client, id: String) -> Result<(), CliError> {
    let thread_pubkey = Thread::pubkey(client.payer_pubkey(), id.into_bytes());
    let ix = Instruction {
        program_id: clockwork_thread_program::ID,
        accounts: clockwork_thread_program::accounts::ThreadReset {
            authority: client.payer_pubkey(),
            thread: thread_pubkey
        }.to_account_metas(Some(false)),
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
    let ix = Instruction {
        program_id: clockwork_thread_program::ID,
        accounts: clockwork_thread_program::accounts::ThreadUpdate {
            authority: client.payer_pubkey(),
            system_program: system_program::ID,
            thread: thread_pubkey
        }.to_account_metas(Some(false)),
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
