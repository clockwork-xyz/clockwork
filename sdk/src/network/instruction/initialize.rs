use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program, sysvar,
    },
    InstructionData,
};

pub fn initialize(
    admin: Pubkey,
    authority: Pubkey,
    config: Pubkey,
    fee: Pubkey,
    mint: Pubkey,
    pool: Pubkey,
    queue: Pubkey,
    registry: Pubkey,
    snapshot: Pubkey,
    task: Pubkey,
) -> Instruction {
    Instruction {
        program_id: cronos_network::ID,
        accounts: vec![
            AccountMeta::new(admin, true),
            AccountMeta::new(authority, false),
            AccountMeta::new_readonly(sysvar::clock::ID, false),
            AccountMeta::new(config, false),
            AccountMeta::new(fee, false),
            AccountMeta::new(mint, false),
            AccountMeta::new(pool, false),
            AccountMeta::new(queue, false),
            AccountMeta::new(registry, false),
            AccountMeta::new_readonly(cronos_scheduler::ID, false),
            AccountMeta::new(snapshot, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new(task, false),
        ],
        data: cronos_network::instruction::Initialize {
            // authority_bump: authority_pda.1,
            // config_bump: config_pda.1,
            // fee_bump: fee_pda.1,
            // pool_bump: pool_pda.1,
            // queue_bump: queue_pda.1,
            // registry_bump: registry_pda.1,
            // snapshot_bump: snapshot_pda.1,
            // task_bump: task_pda.1,
        }
        .data(),
    }
}
