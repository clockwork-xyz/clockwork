use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        sysvar,
    },
    InstructionData,
};

pub fn cycler_run(entry: Pubkey, signer: Pubkey, snapshot: Pubkey) -> Instruction {
    Instruction {
        program_id: cronos_network::ID,
        accounts: vec![
            AccountMeta::new_readonly(sysvar::clock::ID, false),
            AccountMeta::new_readonly(cronos_network::state::Config::pda().0, false),
            AccountMeta::new(cronos_network::state::Cycler::pda().0, false),
            AccountMeta::new_readonly(entry, false),
            AccountMeta::new(cronos_pool::state::Pool::pda().0, false),
            AccountMeta::new_readonly(cronos_pool::state::Config::pda().0, false),
            AccountMeta::new_readonly(cronos_pool::ID, false),
            AccountMeta::new(cronos_network::state::Registry::pda().0, false),
            AccountMeta::new(signer, true),
            AccountMeta::new_readonly(snapshot, false),
        ],
        data: cronos_network::instruction::CyclerRun {}.data(),
    }
}
