use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program,
    },
    InstructionData,
};
use clockwork_network_program::state::{Config, Pool, Registry, Snapshot};

use crate::{client::Client, errors::CliError};

pub fn initialize(client: &Client, mint: Pubkey) -> Result<(), CliError> {
    // Initialize the programs
    let admin = client.payer_pubkey();
    // let ix_a = clockwork_client::network::instruction::initialize(admin, mint);
    // let ix_b = clockwork_client::network::instruction::pool_create(admin, admin, Pool::pubkey(0));
    let ix_a = Instruction {
        program_id: clockwork_network_program::ID,
        accounts: vec![
            AccountMeta::new(admin, true),
            AccountMeta::new(Config::pubkey(), false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new(Registry::pubkey(), false),
            AccountMeta::new(Snapshot::pubkey(0), false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: clockwork_network_program::instruction::Initialize {}.data(),
    };
    let ix_b = Instruction {
        program_id: clockwork_network_program::ID,
        accounts: vec![
            AccountMeta::new_readonly(admin, true),
            AccountMeta::new_readonly(Config::pubkey(), false),
            AccountMeta::new(admin, true),
            AccountMeta::new(Pool::pubkey(0), false),
            AccountMeta::new(Registry::pubkey(), false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: clockwork_network_program::instruction::PoolCreate {}.data(),
    };

    // Submit tx
    client
        .send_and_confirm(&[ix_a, ix_b], &[client.payer()])
        .unwrap();

    Ok(())
}
