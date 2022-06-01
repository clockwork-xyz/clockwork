use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program,
    },
    InstructionData,
};

pub fn initialize(admin: Pubkey) -> Instruction {
    let config = cronos_pool::state::Config::pda().0;
    let cycler = cronos_network::state::Cycler::pda().0;
    let pool = cronos_pool::state::Pool::pda().0;
    Instruction {
        program_id: cronos_pool::ID,
        accounts: vec![
            AccountMeta::new(admin, true),
            AccountMeta::new(config, false),
            AccountMeta::new(pool, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: cronos_pool::instruction::Initialize { cycler }.data(),
    }
}
