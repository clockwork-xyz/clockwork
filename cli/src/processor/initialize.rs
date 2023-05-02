use anchor_lang::{
    solana_program::{
        instruction::Instruction,
        pubkey::Pubkey,
        system_program,
    },
    InstructionData, ToAccountMetas,
};
use clockwork_network_program::state::{Config, Pool, Registry, Snapshot};

use crate::{client::Client, errors::CliError};

pub fn initialize(client: &Client, mint: Pubkey) -> Result<(), CliError> {
    // Initialize the programs
    let admin = client.payer_pubkey();
    let ix_a = Instruction {
        program_id: clockwork_network_program::ID,
        accounts: clockwork_network_program::accounts::Initialize {
            admin,
            config: Config::pubkey(),
            mint,
            registry: Registry::pubkey(),
            snapshot: Snapshot::pubkey(0),
            system_program: system_program::ID,
        }.to_account_metas(Some(false)),
        data: clockwork_network_program::instruction::Initialize {}.data(),
    };
    let ix_b = Instruction {
        program_id: clockwork_network_program::ID,
        accounts: clockwork_network_program::accounts::PoolCreate {
            admin,
            config: Config::pubkey(),
            payer: admin,
            pool: Pool::pubkey(0),
            registry: Registry::pubkey(),
            system_program: system_program::ID,
        }.to_account_metas(Some(false)),
        data: clockwork_network_program::instruction::PoolCreate {}.data(),
    };

    // Submit tx
    client
        .send_and_confirm(&[ix_a, ix_b], &[client.payer()])
        .unwrap();

    Ok(())
}
